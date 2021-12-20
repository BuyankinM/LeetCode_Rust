// 717. 1-bit and 2-bit Characters
// https://leetcode.com/problems/1-bit-and-2-bit-characters/

use crate::Solution;

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let (mut i, last_ind) = (0, bits.len() - 1);
        while i < last_ind {
            i += bits[i] as usize + 1;
        }
        i == last_ind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_one_bit_character(vec![1, 0, 0]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_one_bit_character(vec![1, 1, 1, 0]));
    }

    #[test]
    fn test_3() {
        assert!(Solution::is_one_bit_character(vec![1, 0, 0, 0]));
    }

    #[test]
    fn test_4() {
        assert!(Solution::is_one_bit_character(vec![0, 0]));
    }

    #[test]
    fn test_5() {
        assert!(!Solution::is_one_bit_character(vec![1, 0]));
    }
}
