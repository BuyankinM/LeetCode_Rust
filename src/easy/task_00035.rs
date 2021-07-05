// 35. Search Insert Position
// https://leetcode.com/problems/search-insert-position/

use crate::Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;
        let (mut low, mut high) = (0, nums.len());
        while low < high {
            let mid = (high + low) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => return mid as i32,
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid,
            }
        }
        low as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::search_insert(vec![1, 3, 5, 6], 5));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::search_insert(vec![1, 3, 5, 6], 2));
    }

    #[test]
    fn test_3() {
        assert_eq!(4, Solution::search_insert(vec![1, 3, 5, 6], 7));
    }

    #[test]
    fn test_4() {
        assert_eq!(0, Solution::search_insert(vec![1, 3, 5, 6], 0));
    }

    #[test]
    fn test_5() {
        assert_eq!(0, Solution::search_insert(vec![1], 0));
    }

    #[test]
    fn test_6() {
        assert_eq!(1, Solution::search_insert(vec![1, 3], 2));
    }
}
