// 674. Longest Continuous Increasing Subsequence
// https://leetcode.com/problems/longest-continuous-increasing-subsequence/

use crate::Solution;

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let (mut cur_len, mut max_len) = (1, 1);
        let mut prev_val = nums[0];

        for &val in &nums[1..] {
            match val > prev_val {
                true => cur_len += 1,
                false => {
                    max_len = max_len.max(cur_len);
                    cur_len = 1;
                }
            }
            prev_val = val;
        }
        max_len.max(cur_len)
    }

    // https://leetcode.com/problems/longest-continuous-increasing-subsequence/discuss/1646129/rust-windows
    pub fn find_length_of_lcis_wind(nums: Vec<i32>) -> i32 {
        let mut max_len = 0;
        let accum = nums.windows(2).fold(1, |accum, w| {
            if w[0] < w[1] {
                accum + 1
            } else {
                if accum > max_len {
                    max_len = accum;
                }
                1
            }
        });
        max_len.max(accum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::find_length_of_lcis(vec![2, 2, 2, 2, 2]));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::find_length_of_lcis(vec![2]));
    }

    #[test]
    fn test_4() {
        assert_eq!(1, Solution::find_length_of_lcis(vec![2, 1, 0]));
    }
}
