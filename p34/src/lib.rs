use std::fmt;
use std::ops::{Add, AddAssign, BitAnd, BitOr, BitXor, Mul, MulAssign, Not, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BigUint4096 {
    pub limbs: [u64; 4096 / 64], // 64 limbs
}

impl BigUint4096 {
    /// Zero constant
    pub const ZERO: Self = BigUint4096 { limbs: [0; 64] };

    /// One constant
    pub const ONE: Self = {
        let mut limbs = [0; 64];
        limbs[0] = 1;
        BigUint4096 { limbs }
    };

    /// Create from raw limbs
    pub fn new(limbs: [u64; 64]) -> Self {
        BigUint4096 { limbs }
    }

    /// Creates a new BigUint4096 from a u64 value
    pub fn from_u64(value: u64) -> Self {
        let mut limbs = [0; 64];
        limbs[0] = value;
        BigUint4096 { limbs }
    }

    /// Checks if the value is zero
    pub fn is_zero(&self) -> bool {
        self.limbs.iter().all(|&x| x == 0)
    }

    /// Returns the maximum value (all bits set)
    pub fn max_value() -> Self {
        BigUint4096 { limbs: [u64::MAX; 64] }
    }

    /// Converts the BigUint4096 to a hex string
    pub fn to_hex_string(&self) -> String {
        if self.is_zero() {
            return "0".to_string();
        }

        let mut result = String::new();
        let mut started = false;

        // Process limbs in big-endian order (most significant first)
        for &limb in self.limbs.iter().rev() {
            if started {
                // If we've already started, add all digits with leading zeros
                result.push_str(&format!("{:016x}", limb));
            } else if limb != 0 {
                // Skip leading zeros in the most significant limb
                result.push_str(&format!("{:x}", limb));
                started = true;
            }
        }

        result
    }

    /// Converts a hex string to BigUint4096
    pub fn from_hex_string(hex_str: &str) -> Result<Self, String> {
        let hex_str = hex_str.trim_start_matches("0x").trim_start_matches("0X");
        let mut result = BigUint4096::ZERO;

        // Check if the string is too long (more than 4096 bits, or 1024 hex chars)
        if hex_str.len() > 1024 {
            return Err("Hex string too long for BigUint4096".to_string());
        }

        // Process chunks of 16 characters (64 bits)
        let mut remaining = hex_str;
        let mut limb_index = 0;

        while !remaining.is_empty() {
            // Take up to 16 characters from the end
            let chunk_size = remaining.len().min(16);
            let start = remaining.len() - chunk_size;
            let chunk = &remaining[start..];
            remaining = &remaining[..start];

            // Parse the chunk as a u64
            match u64::from_str_radix(chunk, 16) {
                Ok(value) => {
                    if limb_index >= 64 {
                        return Err("Hex string too large for BigUint4096".to_string());
                    }
                    result.limbs[limb_index] = value;
                    limb_index += 1;
                },
                Err(_) => return Err(format!("Invalid hex digit in '{}'", chunk)),
            }
        }

        Ok(result)
    }

    /// Add with carry
    fn add_with_carry(&self, other: &Self) -> (Self, bool) {
        let mut result = BigUint4096::ZERO;
        let mut carry = false;

        for i in 0..64 {
            let (sum1, overflow1) = self.limbs[i].overflowing_add(other.limbs[i]);
            let (sum2, overflow2) = sum1.overflowing_add(if carry { 1 } else { 0 });

            result.limbs[i] = sum2;
            carry = overflow1 || overflow2;
        }

        (result, carry)
    }

    /// Subtract with borrow
    fn sub_with_borrow(&self, other: &Self) -> (Self, bool) {
        let mut result = BigUint4096::ZERO;
        let mut borrow = false;

        for i in 0..64 {
            let (diff1, underflow1) = self.limbs[i].overflowing_sub(other.limbs[i]);
            let (diff2, underflow2) = diff1.overflowing_sub(if borrow { 1 } else { 0 });

            result.limbs[i] = diff2;
            borrow = underflow1 || underflow2;
        }

        (result, borrow)
    }

    /// Shifts left by a number of bits, returning the result
    pub fn shl_bits(&self, shift: usize) -> Self {
        let mut result = BigUint4096::ZERO;

        // Special case for zero shift
        if shift == 0 {
            return *self;
        }

        // Handle shifts larger than or equal to 4096 bits
        if shift >= 4096 {
            return BigUint4096::ZERO;
        }

        // Calculate the limb offset and bit offset
        let limb_shift = shift / 64;
        let bit_shift = shift % 64;

        if bit_shift == 0 {
            // Simple case: just shift limbs
            for i in limb_shift..64 {
                result.limbs[i] = self.limbs[i - limb_shift];
            }
        } else {
            // Complex case: need to handle bits crossing limb boundaries
            for i in limb_shift..64 {
                // Get bits from current limb
                let current = if i - limb_shift < 64 {
                    self.limbs[i - limb_shift] << bit_shift
                } else {
                    0
                };

                // Get bits from previous limb if possible
                let previous = if i > limb_shift && i - limb_shift - 1 < 64 {
                    self.limbs[i - limb_shift - 1] >> (64 - bit_shift)
                } else {
                    0
                };

                result.limbs[i] = current | previous;
            }
        }

        result
    }

    /// Shifts right by a number of bits, returning the result
    pub fn shr_bits(&self, shift: usize) -> Self {
        let mut result = BigUint4096::ZERO;

        // Special case for zero shift
        if shift == 0 {
            return *self;
        }

        // Handle shifts larger than or equal to 4096 bits
        if shift >= 4096 {
            return BigUint4096::ZERO;
        }

        // Calculate the limb offset and bit offset
        let limb_shift = shift / 64;
        let bit_shift = shift % 64;

        if bit_shift == 0 {
            // Simple case: just shift limbs
            for i in 0..(64 - limb_shift) {
                result.limbs[i] = self.limbs[i + limb_shift];
            }
        } else {
            // Complex case: need to handle bits crossing limb boundaries
            for i in 0..(64 - limb_shift) {
                // Get bits from current limb
                let current = self.limbs[i + limb_shift] >> bit_shift;

                // Get bits from next limb if possible
                let next = if i + limb_shift + 1 < 64 {
                    self.limbs[i + limb_shift + 1] << (64 - bit_shift)
                } else {
                    0
                };

                result.limbs[i] = current | next;
            }
        }

        result
    }
}

// Implement FromStr trait to allow parsing from strings
impl FromStr for BigUint4096 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(stripped) = s.strip_prefix("0x") {
            BigUint4096::from_hex_string(stripped)
        } else {
            // For simplicity, we'll interpret all strings as hex
            // In a more complete implementation, you might want to handle decimal strings as well
            BigUint4096::from_hex_string(s)
        }
    }
}

// Display implementation (outputs as hex)
impl fmt::Display for BigUint4096 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", self.to_hex_string())
    }
}

// Implementation of Add for BigUint4096
impl Add for BigUint4096 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (result, _) = self.add_with_carry(&other);
        result
    }
}

// Implementation of AddAssign for BigUint4096
impl AddAssign for BigUint4096 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

// Implementation of Sub for BigUint4096
impl Sub for BigUint4096 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (result, _) = self.sub_with_borrow(&other);
        result
    }
}

// Implementation of SubAssign for BigUint4096
impl SubAssign for BigUint4096 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

// Implementation of BitAnd for BigUint4096
impl BitAnd for BigUint4096 {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        let mut result = BigUint4096::ZERO;
        for i in 0..64 {
            result.limbs[i] = self.limbs[i] & other.limbs[i];
        }
        result
    }
}

// Implementation of BitOr for BigUint4096
impl BitOr for BigUint4096 {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        let mut result = BigUint4096::ZERO;
        for i in 0..64 {
            result.limbs[i] = self.limbs[i] | other.limbs[i];
        }
        result
    }
}

// Implementation of BitXor for BigUint4096
impl BitXor for BigUint4096 {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        let mut result = BigUint4096::ZERO;
        for i in 0..64 {
            result.limbs[i] = self.limbs[i] ^ other.limbs[i];
        }
        result
    }
}

// Implementation of Not for BigUint4096
impl Not for BigUint4096 {
    type Output = Self;

    fn not(self) -> Self {
        let mut result = BigUint4096::ZERO;
        for i in 0..64 {
            result.limbs[i] = !self.limbs[i];
        }
        result
    }
}

// Implementation of Shl for BigUint4096
impl Shl<usize> for BigUint4096 {
    type Output = Self;

    fn shl(self, shift: usize) -> Self {
        self.shl_bits(shift)
    }
}

// Implementation of ShlAssign for BigUint4096
impl ShlAssign<usize> for BigUint4096 {
    fn shl_assign(&mut self, shift: usize) {
        *self = self.shl_bits(shift);
    }
}

// Implementation of Shr for BigUint4096
impl Shr<usize> for BigUint4096 {
    type Output = Self;

    fn shr(self, shift: usize) -> Self {
        self.shr_bits(shift)
    }
}

// Implementation of ShrAssign for BigUint4096
impl ShrAssign<usize> for BigUint4096 {
    fn shr_assign(&mut self, shift: usize) {
        *self = self.shr_bits(shift);
    }
}

// Basic multiplication implementation (this is a naive implementation for clarity)
// For a real-world implementation, you'd want to use a more efficient algorithm like Karatsuba
impl Mul for BigUint4096 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        // Start with zero
        let mut result = BigUint4096::ZERO;

        // For each bit in other
        for i in 0..64 {
            for bit in 0..64 {
                if (other.limbs[i] & (1u64 << bit)) != 0 {
                    // If the bit is set, add self shifted by the bit position
                    let shift_amount = i * 64 + bit;
                    let shifted = self.shl_bits(shift_amount);
                    result = result + shifted;
                }
            }
        }

        result
    }
}

// Implementation of MulAssign for BigUint4096
impl MulAssign for BigUint4096 {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

// Add tests for the BigUint4096 implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let zero = BigUint4096::ZERO;
        assert!(zero.is_zero());
    }

    #[test]
    fn test_one() {
        let one = BigUint4096::ONE;
        assert_eq!(one.limbs[0], 1);
        for i in 1..64 {
            assert_eq!(one.limbs[i], 0);
        }
    }

    #[test]
    fn test_from_u64() {
        let value = BigUint4096::from_u64(123456789);
        assert_eq!(value.limbs[0], 123456789);
        for i in 1..64 {
            assert_eq!(value.limbs[i], 0);
        }
    }

    #[test]
    fn test_to_hex_string() {
        let mut value = BigUint4096::ZERO;
        value.limbs[0] = 0xabcdef;
        value.limbs[1] = 0x123456;
        assert_eq!(value.to_hex_string(), "123456abcdef");
    }

    #[test]
    fn test_from_hex_string() {
        let value = BigUint4096::from_hex_string("123456abcdef").unwrap();
        assert_eq!(value.limbs[0], 0xabcdef);
        assert_eq!(value.limbs[1], 0x123456);
        for i in 2..64 {
            assert_eq!(value.limbs[i], 0);
        }
    }

    #[test]
    fn test_add() {
        let a = BigUint4096::from_u64(0xFFFFFFFFFFFFFFFF); // All bits set in limb 0
        let b = BigUint4096::from_u64(1); // Just 1

        let sum = a + b;
        assert_eq!(sum.limbs[0], 0);
        assert_eq!(sum.limbs[1], 1);
        for i in 2..64 {
            assert_eq!(sum.limbs[i], 0);
        }
    }

    #[test]
    fn test_sub() {
        let a = BigUint4096::from_u64(100);
        let b = BigUint4096::from_u64(50);

        let diff = a - b;
        assert_eq!(diff.limbs[0], 50);
        for i in 1..64 {
            assert_eq!(diff.limbs[i], 0);
        }
    }

    #[test]
    fn test_bitwise_operations() {
        let a = BigUint4096::from_u64(0b1010);
        let b = BigUint4096::from_u64(0b1100);

        // AND
        let and_result = a & b;
        assert_eq!(and_result.limbs[0], 0b1000);

        // OR
        let or_result = a | b;
        assert_eq!(or_result.limbs[0], 0b1110);

        // XOR
        let xor_result = a ^ b;
        assert_eq!(xor_result.limbs[0], 0b0110);

        // NOT
        let not_result = !a;
        assert_eq!(not_result.limbs[0], !0b1010);
    }

    #[test]
    fn test_shifts() {
        let a = BigUint4096::from_u64(0x1);

        // Left shift within a limb
        let left_shift = a << 10;
        assert_eq!(left_shift.limbs[0], 0x1 << 10);

        // Left shift across limbs
        let left_shift_across = a << 64;
        assert_eq!(left_shift_across.limbs[0], 0);
        assert_eq!(left_shift_across.limbs[1], 0x1);

        // Create a value with bits set in limb 1
        let mut b = BigUint4096::ZERO;
        b.limbs[1] = 0x8000_0000_0000_0000;

        // Right shift across limbs
        let right_shift_across = b >> 1;
        assert_eq!(right_shift_across.limbs[0], 0x8000_0000_0000_0000);
        assert_eq!(right_shift_across.limbs[1], 0x4000_0000_0000_0000);
    }

    #[test]
    fn test_multiplication() {
        let a = BigUint4096::from_u64(123);
        let b = BigUint4096::from_u64(456);

        let product = a * b;
        assert_eq!(product.limbs[0], 123 * 456);
        for i in 1..64 {
            assert_eq!(product.limbs[i], 0);
        }

        // Test multiplication involving carry
        let c = BigUint4096::from_u64(0xFFFFFFFFFFFFFFFF);
        let d = BigUint4096::from_u64(2);

        let big_product = c * d;
        assert_eq!(big_product.limbs[0], 0xFFFFFFFFFFFFFFFE);
        assert_eq!(big_product.limbs[1], 1);
    }
}