// 413. Arithmetic Slices
// https://leetcode.com/problems/arithmetic-slices/

use crate::Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }

        let calc_num = |l: i32| -> i32 {
            match l >= 3 {
                true => (l - 2) * (l - 1) / 2, // sum(1..n) = n * (n + 1) / 2
                false => 0,
            }
        };
        let mut res = 0;
        let mut prev_delta = nums[1] - nums[0];
        let mut prev_x = nums[1];
        let mut l = 2;

        for &x in &nums[2..] {
            let delta = x - prev_x;
            match delta == prev_delta {
                true => l += 1,
                false => {
                    res += calc_num(l);
                    prev_delta = delta;
                    l = 2;
                }
            }
            prev_x = x;
        }
        res + calc_num(l)
    }

    // https://leetcode.com/problems/arithmetic-slices/discuss/1815023/Rust-DP-with-constant-space
    pub fn number_of_arithmetic_slices_short(nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        let mut cnt: i32 = 0;
        let mut cur: i32 = 0;
        for idx in 2..len_n {
            if nums[idx] - nums[idx - 1] == nums[idx - 1] - nums[idx - 2] {
                cur += 1;
                cnt += cur;
            } else {
                cur = 0;
            }
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::number_of_arithmetic_slices(vec![1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            4,
            Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4, 9, 10, 11])
        );
    }
}
