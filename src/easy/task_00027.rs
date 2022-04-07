// 27. Remove Element
// https://leetcode.com/problems/remove-element/

use crate::Solution;

impl Solution {
    pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
        let l = nums.len();
        let mut res = 0;
        let mut min_ind: i32 = -1;

        for i in 0..l {
            if nums[i] == val {
                res += 1;
                if min_ind == -1 {
                    min_ind = i as i32
                }
            } else if min_ind > -1 {
                nums.swap(i, min_ind as usize);
                min_ind += 1;
            }
        }

        (l - res) as i32
    }

    pub fn remove_element_optimal(nums: &mut [i32], val: i32) -> i32 {
        let (mut len, mut i) = (nums.len(), 0);
        while i < len {
            if nums[i] == val {
                nums.swap(i, len - 1);
                len -= 1;
            } else {
                i += 1;
            }
        }
        len as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::remove_element(&mut [3, 2, 2, 3], 3));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            5,
            Solution::remove_element_optimal(&mut [0, 1, 2, 2, 3, 0, 4, 2], 2)
        );
    }
}
