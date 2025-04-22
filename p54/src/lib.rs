#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]

use core::arch::x86_64::*;

const AES_ROUNDS: usize = 10;

/// Expand a 128-bit AES key into 11 round keys
#[target_feature(enable = "aes")]
unsafe fn expand_key(key: &[u8; 16]) -> [__m128i; AES_ROUNDS + 1] {
    let mut rk: [__m128i; AES_ROUNDS + 1] = [_mm_setzero_si128(); AES_ROUNDS + 1];
    let mut tmp = _mm_loadu_si128(key.as_ptr() as *const __m128i);
    rk[0] = tmp;
    macro_rules! round {
        ($i:expr, $r:expr) => {{
            let keygen = _mm_aeskeygenassist_si128(tmp, $r);
            let mut keygen2 = _mm_shuffle_epi32(keygen, 0xff);
            tmp = _mm_xor_si128(tmp, _mm_slli_si128(tmp, 4));
            tmp = _mm_xor_si128(tmp, _mm_slli_si128(tmp, 4));
            tmp = _mm_xor_si128(tmp, _mm_slli_si128(tmp, 4));
            tmp = _mm_xor_si128(tmp, keygen2);
            rk[$i] = tmp;
        }};
    }
    round!(1, 0x01);
    round!(2, 0x02);
    round!(3, 0x04);
    round!(4, 0x08);
    round!(5, 0x10);
    round!(6, 0x20);
    round!(7, 0x40);
    round!(8, 0x80);
    round!(9, 0x1B);
    round!(10, 0x36);
    rk
}

/// Encrypt a single 128-bit block in place
#[target_feature(enable = "aes")]
unsafe fn encrypt1(rk: &[__m128i; AES_ROUNDS + 1], block: &mut [u8; 16]) {
    let mut s = _mm_loadu_si128(block.as_ptr() as *const __m128i);
    s = _mm_xor_si128(s, rk[0]);
    for i in 1..AES_ROUNDS {
        s = _mm_aesenc_si128(s, rk[i]);
    }
    s = _mm_aesenclast_si128(s, rk[AES_ROUNDS]);
    _mm_storeu_si128(block.as_mut_ptr() as *mut __m128i, s);
}

/// Apply AES-CTR keystream (64-bit counter, nonce = 0) to `data` in place
pub fn apply_keystream(key: &[u8; 16], data: &mut [u8]) {
    assert!(is_x86_feature_detected!("aes"));
    unsafe {
        let rk = expand_key(key);
        let mut counter: u64 = 0;
        let mut ctr_block = [0u8; 16];
        for chunk in data.chunks_mut(16) {
            // nonce (upper 64 bits) = 0, counter (lower) big-endian
            ctr_block[..8].fill(0);
            ctr_block[8..].copy_from_slice(&counter.to_be_bytes());
            let mut blk = ctr_block;
            encrypt1(&rk, &mut blk);
            for i in 0..chunk.len() {
                chunk[i] ^= blk[i];
            }
            counter = counter.wrapping_add(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ctr_zero_keystream() {
        // With key=0, AES-CTR of zero data flips to AES(ctr)
        let key = [0u8; 16];
        let mut data = [0u8; 32];
        apply_keystream(&key, &mut data);
        // First block = AES(ctr=0), second = AES(ctr=1)
        assert_ne!(data[0..16], [0u8; 16]);
        assert_ne!(data[16..32], [0u8; 16]);
    }

    #[test]
    fn ctr_roundtrip() {
        let key = [0x42u8; 16];
        let mut data = b"Hello, AES-CTR with AES-NI!   ".to_vec();
        apply_keystream(&key, &mut data);
        let mut encrypted = data.clone();
        apply_keystream(&key, &mut data);
        assert_eq!(data, encrypted.iter().map(|b| !b).collect::<Vec<u8>>());
    }
}
