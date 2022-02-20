// 2176. Count Equal and Divisible Pairs in an Array
// https://leetcode.com/problems/count-equal-and-divisible-pairs-in-an-array/

use crate::Solution;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let mut nums_ind = nums.iter().cloned().zip(0..).collect::<Vec<_>>();
        nums_ind.sort_unstable_by_key(|(x, _)| *x);

        for (ind, &(x, i)) in nums_ind.iter().enumerate() {
            for &(y, j) in nums_ind[ind + 1..].iter() {
                match y > x {
                    true => break,
                    false if i * j % k == 0 => res += 1,
                    _ => (),
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::count_pairs(vec![1, 2, 3, 4], 1));
    }
}
