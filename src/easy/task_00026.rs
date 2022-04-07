// 26. Remove Duplicates from Sorted Array
// https://leetcode.com/problems/remove-duplicates-from-sorted-array/

use crate::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
        match nums.len() {
            0 => return 0,
            1 => return 1,
            _ => (),
        }

        let mut min_ind: usize = 1;
        let mut val = nums[0];

        for i in 1..nums.len() {
            if val != nums[i] {
                val = nums[i];
                nums.swap(i, min_ind);
                min_ind += 1;
            }
        }

        min_ind as i32
    }

    pub fn remove_duplicates_best(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            5,
            Solution::remove_duplicates(&mut [0, 0, 1, 1, 1, 2, 2, 3, 3, 4])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::remove_duplicates_best(&mut vec![1, 1, 2]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            5,
            Solution::remove_duplicates_best(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4])
        );
    }
}
