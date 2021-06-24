// 1748. Sum of Unique Elements
// https://leetcode.com/problems/sum-of-unique-elements/

use crate::Solution;

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold([0; 101], |mut acc, &val| {
                acc[val as usize] += 1;
                acc
            })
            .iter()
            .enumerate()
            .filter_map(|(ind, &x)| if x == 1 { Some(ind as i32) } else { None })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::sum_of_unique(vec![1, 2, 3, 2]));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::sum_of_unique(vec![1, 1, 1, 1, 1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(15, Solution::sum_of_unique(vec![1, 2, 3, 4, 5]));
    }
}
