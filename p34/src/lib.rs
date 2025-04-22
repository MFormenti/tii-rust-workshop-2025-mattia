#![deny(warnings)]
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct BigUint<const N: usize>(pub [u64; N]);
pub type BigUint4096 = BigUint<64>;

impl<const N: usize> BigUint<N> {
    pub fn zero() -> Self {
        Self([0; N])
    }

    pub fn from_hex_be(s: &str) -> Result<Self, String> {
        let mut result = [0u64; N];
        let hex = s.trim_start_matches("0x");
        let needed = N * 16;
        let padded = if hex.len() < needed {
            let mut p = String::with_capacity(needed);
            for _ in 0..(needed - hex.len()) {
                p.push('0');
            }
            p.push_str(hex);
            p
        } else if hex.len() > needed {
            return Err("Hex string too long".into());
        } else {
            hex.to_string()
        };
        for i in 0..N {
            let start = needed - (i + 1) * 16;
            let part = &padded[start..start + 16];
            result[i] = u64::from_str_radix(part, 16)
                .map_err(|e| format!("Invalid hex at limb {}: {}", i, e))?;
        }
        Ok(Self(result))
    }

    pub fn to_hex_be(&self) -> String {
        let mut buf = String::with_capacity(N * 16);
        for &limb in self.0.iter().rev() {
            buf.push_str(&format!("{:016X}", limb));
        }
        buf
    }
}

impl<const N: usize> Add for BigUint<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = [0u64; N];
        let mut carry = 0u128;
        for i in 0..N {
            let sum = (self.0[i] as u128) + (rhs.0[i] as u128) + carry;
            res[i] = sum as u64;
            carry = sum >> 64;
        }
        BigUint(res)
    }
}
impl<const N: usize> AddAssign for BigUint<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const N: usize> Sub for BigUint<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = [0u64; N];
        let mut borrow = 0i128;
        for i in 0..N {
            let diff = (self.0[i] as i128) - (rhs.0[i] as i128) - borrow;
            if diff < 0 {
                res[i] = (diff + (1i128 << 64)) as u64;
                borrow = 1;
            } else {
                res[i] = diff as u64;
                borrow = 0;
            }
        }
        BigUint(res)
    }
}
impl<const N: usize> SubAssign for BigUint<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const N: usize> fmt::Display for BigUint<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", self.to_hex_be())
    }
}

impl<const N: usize> FromStr for BigUint<N> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BigUint::from_hex_be(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_simple() {
        let a = BigUint4096::from_hex_be("1").unwrap();
        let b = BigUint4096::from_hex_be("2").unwrap();
        let c = a + b;
        assert_eq!(c.to_hex_be(), format!("{:0>64X}", 3u64));
    }

    #[test]
    fn test_sub_simple() {
        let a = BigUint4096::from_hex_be("10").unwrap();
        let b = BigUint4096::from_hex_be("1").unwrap();
        let c = a - b;
        assert_eq!(c.to_hex_be(), format!("{:0>64X}", 15u64));
    }

    #[test]
    fn test_overflow_add() {
        let max = BigUint4096([!0u64; 64]);
        let one = BigUint4096::from_hex_be("1").unwrap();
        let sum = max + one;
        assert_eq!(sum.0[0], 0);
    }

    #[test]
    fn test_display_and_parse() {
        let hex = "123456789ABCDEF0FEDCBA9876543210";
        let s = format!("0x{}", hex);
        let x: BigUint4096 = s.parse().unwrap();
        assert_eq!(x.to_hex_be()[64*16-hex.len()..], hex);
        assert_eq!(x.to_string(), format!("0x{}", x.to_hex_be()));
    }
}