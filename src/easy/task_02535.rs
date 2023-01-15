// 2535. Difference Between Element Sum and Digit Sum of an Array
// https://leetcode.com/problems/difference-between-element-sum-and-digit-sum-of-an-array/

use crate::Solution;

impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let sum_dig = |mut x: i32| {
            let mut res = 0;
            while x > 0 {
                res += x % 10;
                x /= 10;
            }
            res
        };
        nums.into_iter().map(|x| (x - sum_dig(x)).abs()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(9, Solution::difference_of_sum(vec![1, 15, 6, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::difference_of_sum(vec![1, 2, 3, 4]));
    }
}
