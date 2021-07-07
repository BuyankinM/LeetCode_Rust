// 1295. Find Numbers with Even Number of Digits
// https://leetcode.com/problems/find-numbers-with-even-number-of-digits/

use crate::Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.iter()
            .filter(|&&elem| (((elem as f32).log10().floor() + 1.0) as u32) % 2 == 0)
            .count() as i32
    }

    pub fn find_numbers_optimal(nums: Vec<i32>) -> i32 {
        nums.iter()
            .filter(|&&elem| (elem as f32).log10() as u32 % 2 == 1)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::find_numbers(vec![12, 345, 2, 6, 7896]));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::find_numbers_optimal(vec![555, 901, 482, 1771]));
    }
}
