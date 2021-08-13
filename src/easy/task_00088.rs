// 88. Merge Sorted Array
// https://leetcode.com/problems/merge-sorted-array/

use crate::Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if n == 0 {
            return;
        }

        let mut mm = m - 1;
        let mut nn = n - 1;

        while nn >= 0 && mm >= 0 {
            let main_ind = (mm + nn + 1) as usize;

            if nums1[mm as usize] > nums2[nn as usize] {
                nums1[main_ind] = nums1[mm as usize];
                mm -= 1;
            } else {
                nums1[main_ind] = nums2[nn as usize];
                nn -= 1;
            }
        }

        if nn >= 0 {
            for i in 0..(nn + 1) {
                nums1[i as usize] = nums2[i as usize]
            }
        }
    }

    pub fn merge_best(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        nums1.drain(m as usize..);
        nums2.drain(n as usize..);
        nums1.append(nums2);
        nums1.sort_unstable();
    }

    pub fn merge_order(nums1: &mut Vec<i32>, mut m: i32, nums2: &mut Vec<i32>, mut n: i32) {
        for i in (0..nums1.len()).rev() {
            let a = nums1.get((m - 1) as usize).unwrap_or(&std::i32::MIN);
            let b = nums2.get((n - 1) as usize).unwrap_or(&std::i32::MIN);
            match a.cmp(b) {
                std::cmp::Ordering::Greater => {
                    nums1[i] = *a;
                    m -= 1
                }
                _ => {
                    nums1[i] = *b;
                    n -= 1
                }
            }
        }
    }

    pub fn merge_split_off(nums1: &mut Vec<i32>, _m: i32, nums2: &mut Vec<i32>, n: i32) {
        nums1.truncate(nums1.len() - n as usize);
        nums1.append(nums2);
        nums1.sort_unstable();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        Solution::merge(&mut nums1, 3, &mut vec![2, 5, 6], 3);
        assert_eq!(vec![1, 2, 2, 3, 5, 6], nums1);
    }

    #[test]
    fn test_2() {
        let mut nums1 = vec![1];
        Solution::merge_best(&mut nums1, 1, &mut vec![], 0);
        assert_eq!(vec![1], nums1);
    }

    #[test]
    fn test_3() {
        let mut nums1 = vec![0];
        Solution::merge_split_off(&mut nums1, 0, &mut vec![1], 1);
        assert_eq!(vec![1], nums1);
    }
}
