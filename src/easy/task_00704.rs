// 704. Binary Search
// https://leetcode.com/problems/binary-search/

use crate::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;
        let (mut low, mut high) = (0, nums.len());
        while low < high {
            let mid = low + (high - low) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => return mid as i32,
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid,
            }
        }
        -1
    }

    pub fn search_std(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(i) => i as i32,
            _ => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::search(vec![-1, 0, 3, 5, 9, 12], 9));
    }

    #[test]
    fn test_2() {
        assert_eq!(-1, Solution::search(vec![-1, 0, 3, 5, 9, 12], 2));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::search(vec![-1, 0, 3, 5, 9, 12], -1));
    }
}
