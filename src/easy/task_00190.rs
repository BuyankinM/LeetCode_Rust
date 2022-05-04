// 190. Reverse Bits
// https://leetcode.com/problems/reverse-bits/

use crate::Solution;

impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut res = 0;
        for _ in 0..32 {
            res <<= 1;
            res |= x & 1;
            x >>= 1;
        }
        res
    }

    pub fn reverse_bits_oneliner(x: u32) -> u32 {
        x.reverse_bits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::reverse_bits(0b00000010100101000001111010011100),
            0b00111001011110000010100101000000
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::reverse_bits(0b11111111111111111111111111111101),
            0b10111111111111111111111111111111
        );
    }
}
