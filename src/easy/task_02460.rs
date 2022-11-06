// 2460. Apply Operations to an Array
// https://leetcode.com/problems/apply-operations-to-an-array/

use crate::Solution;

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let l = nums.len();
        (0..l - 1).for_each(|i| {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        });

        let mut pos_zero = 0;
        (0..l).for_each(|i| {
            if nums[i] != 0 {
                if pos_zero < i {
                    nums.swap(i, pos_zero);
                }
                pos_zero += 1;
            }
        });
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::apply_operations(vec![1, 2, 2, 1, 1, 0]),
            vec![1, 4, 2, 0, 0, 0]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::apply_operations(vec![0, 1]), vec![1, 0]);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::apply_operations(vec![
                847, 847, 0, 0, 0, 399, 416, 416, 879, 879, 206, 206, 206, 272
            ]),
            vec![1694, 399, 832, 1758, 412, 206, 272, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
