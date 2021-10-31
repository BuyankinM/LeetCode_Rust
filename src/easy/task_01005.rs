// 1005. Maximize Sum Of Array After K Negations
// https://leetcode.com/problems/maximize-sum-of-array-after-k-negations/

use std::iter::FromIterator;

use crate::Solution;

impl Solution {
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, mut k: i32) -> i32 {
        let mut res = 0;
        let mut min_x = i32::MAX;

        nums.sort_unstable();
        nums.iter().cloned().for_each(|x| {
            let mut val = x;
            if k > 0 {
                if x < 0 {
                    val *= -1;
                    k -= 1;
                } else {
                    if x > 0 && k % 2 == 1 {
                        if x.abs() < min_x {
                            val *= -1;
                        } else {
                            res -= 2 * min_x;
                        }
                    }
                    k = 0;
                }
            }
            min_x = min_x.min(x.abs());
            res += val
        });

        if k % 2 == 1 {
            res -= 2 * min_x;
        }

        res
    }

    pub fn largest_sum_after_k_negations_heap(nums: Vec<i32>, mut k: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let nums_rev = nums.iter().cloned().map(Reverse).collect::<Vec<_>>();
        let mut min_heap = BinaryHeap::from(nums_rev);

        while k > 0 {
            if let Some(Reverse(val)) = min_heap.pop() {
                min_heap.push(Reverse(-val));
            }
            k -= 1;
        }

        min_heap.into_iter().fold(0, |acc, rv| {
            let Reverse(val) = rv;
            acc + val
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::largest_sum_after_k_negations(vec![4, 2, 3], 1));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            5,
            Solution::largest_sum_after_k_negations(vec![-4, -2, -3], 4)
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            6,
            Solution::largest_sum_after_k_negations(vec![3, -1, 0, 2], 3)
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            13,
            Solution::largest_sum_after_k_negations(vec![2, -3, -1, 5, -4], 2)
        );
    }
    
    #[test]
    fn test_5() {
        assert_eq!(
            13,
            Solution::largest_sum_after_k_negations_heap(vec![2, -3, -1, 5, -4], 2)
        );
    }
}
