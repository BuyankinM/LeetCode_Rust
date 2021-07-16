// 1403. Minimum Subsequence in Non-Increasing Order
// https://leetcode.com/problems/minimum-subsequence-in-non-increasing-order/

use crate::Solution;

impl Solution {
    pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        let all_sum: i32 = nums.iter().sum();
        let mut cur_sum = 0;
        let mut result = Vec::with_capacity(nums.len());

        nums.sort_unstable_by_key(|&x| -x);

        for x in nums {
            cur_sum += x;
            result.push(x);
            if cur_sum > all_sum - cur_sum {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![10, 9], Solution::min_subsequence(vec![4, 3, 10, 9, 8]));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![7, 7, 6],
            Solution::min_subsequence(vec![4, 4, 7, 6, 7])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![6], Solution::min_subsequence(vec![6]));
    }
}
