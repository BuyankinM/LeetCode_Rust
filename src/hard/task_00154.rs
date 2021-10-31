// 154. Find Minimum in Rotated Sorted Array II
// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/

use crate::Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        nums.iter().cloned().min().unwrap()
    }

    pub fn find_min_bs(nums: Vec<i32>) -> i32 {
        use std::cmp::Ordering::{Equal, Greater, Less};

        let (mut low, mut high) = (0, nums.len() - 1);
        while low < high {
            let mid = low + ((high - low) / 2);
            match nums[mid].cmp(&nums[high]) {
                Greater => low = mid + 1,
                Less => high = mid,
                Equal => high -= 1, // special case
            };
        }
        nums[low]
    }
}
