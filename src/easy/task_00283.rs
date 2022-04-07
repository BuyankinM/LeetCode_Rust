// 283. Move Zeroes
// https://leetcode.com/problems/move-zeroes/

use crate::Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut [i32]) {
        let mut zero_pos: usize = 0;
        let mut num_zeros = 0;

        for i in 0..nums.len() {
            let val = nums[i];
            if val == 0 {
                if num_zeros == 0 {
                    zero_pos = i
                }
                num_zeros += 1;
            } else if num_zeros > 0 {
                nums[zero_pos] = val;
                nums[i] = 0;
                zero_pos += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut v = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut v);
        assert_eq!(vec![1, 3, 12, 0, 0], v);
    }

    #[test]
    fn test_2() {
        let mut v = vec![0];
        Solution::move_zeroes(&mut v);
        assert_eq!(vec![0], v);
    }
}
