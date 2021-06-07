// 376. Wiggle Subsequence
// https://leetcode.com/problems/wiggle-subsequence/

use crate::Solution;
use std::cmp::Ordering;

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut prev = None;
        let mut answer = 1;
        for i in 1..nums.len() {
            let o = (nums[i] - nums[i - 1]).cmp(&0);
            if o == Ordering::Equal {
                continue;
            }
            if Some(o) != prev {
                answer += 1
            }
            prev = Some(o);
        }
        answer
    }
}
