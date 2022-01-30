// 2154. Keep Multiplying Found Values by Two
// https://leetcode.com/problems/keep-multiplying-found-values-by-two/

use crate::Solution;

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut powers = [false; 11];
        nums.iter().filter(|&&x| x % original == 0).for_each(|&x| {
            let n = (x / original) as u32;
            if n & (n - 1) == 0 {
                powers[n.trailing_zeros() as usize] = true;
            }
        });
        original * (1 << (powers.iter().position(|&x| !x).unwrap_or_default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(24, Solution::find_final_value(vec![5, 3, 6, 1, 12], 3));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, Solution::find_final_value(vec![2, 7, 9], 4));
    }
}
