// 1588. Sum of All Odd Length Subarrays
// https://leetcode.com/problems/sum-of-all-odd-length-subarrays/

use crate::Solution;

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in (1..=arr.len()).step_by(2) {
            for w in arr.windows(i) {
                res += w.iter().sum::<i32>();
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(58, Solution::sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::sum_odd_length_subarrays(vec![1, 2]));
    }

    #[test]
    fn test_3() {
        assert_eq!(66, Solution::sum_odd_length_subarrays(vec![10, 11, 12]));
    }
}
