// 540. Single Element in a Sorted Array
// https://leetcode.com/problems/single-element-in-a-sorted-array/

use crate::Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, &x| acc ^ x)
    }

    pub fn single_non_duplicate_bs(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let (mut low, mut high) = (0, l - 1);
        while low < high {
            let m = low + (high - low) / 2;

            let cur = nums[m];
            let prev = nums[m - ((m > 0) as usize)];
            let next = nums[m + ((m < l - 1) as usize)];
            let correct_pos = (m % 2 == 0) == (cur == next && cur > prev);

            match correct_pos {
                false if cur > prev && cur < next => return nums[m],
                false => high = m - (m > 0) as usize,
                true => low = m + 1,
            }
        }
        nums[low]
    }

    // https://leetcode.com/problems/single-element-in-a-sorted-array/discuss/628686/Rust-0ms
    pub fn single_non_duplicate_bs_recur(nums: Vec<i32>) -> i32 {
        fn bin_search(n: &[i32], lo: usize, hi: usize) -> i32 {
            if lo == hi {
                return n[lo];
            }

            let mid = lo + (hi - lo) / 2;
            match mid % 2 {
                0 if n[mid] == n[mid - 1] => bin_search(n, lo, mid - 2),
                0 if n[mid] == n[mid + 1] => bin_search(n, mid + 2, hi),
                1 if n[mid] == n[mid - 1] => bin_search(n, mid + 1, hi),
                1 if n[mid] == n[mid + 1] => bin_search(n, lo, mid - 1),
                _ => n[mid],
            }
        }

        bin_search(&nums, 0, nums.len() - 1)
    }

    // https://leetcode.com/problems/single-element-in-a-sorted-array/discuss/628369/Rust-binary-search-solution
    pub fn single_non_duplicate_bs_short(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let m = l + (r - l) / 2;
            if (m % 2 == 0) == (nums[m] != nums[m + 1]) {
                r = m;
            } else {
                l = m + 1;
            }
        }
        nums[l]
    }

    // https://leetcode.com/problems/single-element-in-a-sorted-array/discuss/628477/Rust-7-lines-solution-with-explanation-0ms
    pub fn single_non_duplicate_bs_xor(nums: Vec<i32>) -> i32 {
        let mut mid;
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo < hi {
            mid = lo + (hi - lo) / 2;
            match nums[mid] == nums[mid ^ 1] {
                true => lo = mid + 1,
                false => hi = mid,
            }
        }
        nums[lo]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::single_non_duplicate_bs(vec![1, 2, 2, 3, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::single_non_duplicate_bs(vec![1, 1, 2, 3, 3]));
    }

    #[test]
    fn test_3() {
        assert_eq!(3, Solution::single_non_duplicate_bs(vec![1, 1, 2, 2, 3]));
    }
}
