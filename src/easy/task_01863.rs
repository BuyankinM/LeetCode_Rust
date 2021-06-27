// 1863. Sum of All Subset XOR Totals
// https://leetcode.com/problems/sum-of-all-subset-xor-totals/

use crate::Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut res = 0;

        for comb in 1..1 << l {
            let mut val_xor = 0;
            for i in 0..l {
                let mask = 1 << i;
                if comb & mask != 0 {
                    val_xor ^= nums[i as usize];
                }
            }
            res += val_xor;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::subset_xor_sum(vec![1, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(28, Solution::subset_xor_sum(vec![5, 1, 6]));
    }

    #[test]
    fn test_3() {
        assert_eq!(480, Solution::subset_xor_sum(vec![3, 4, 5, 6, 7, 8]));
    }
}
