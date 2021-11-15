// 922. Sort Array By Parity II
// https://leetcode.com/problems/sort-array-by-parity-ii/

use crate::Solution;

impl Solution {
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        let (mut evens, mut odds): (Vec<_>, Vec<_>) = nums.iter().partition(|&n| n % 2 == 0);
        nums.iter_mut().enumerate().for_each(|(ind, val)| {
            *val = match ind & 1 {
                1 => odds.pop().unwrap(),
                _ => evens.pop().unwrap(),
            }
        });
        nums
    }

    // https://leetcode.com/problems/sort-array-by-parity-ii/discuss/1491765/Rust-fp-and-inplace
    pub fn sort_array_by_parity_ii_fp(nums: Vec<i32>) -> Vec<i32> {
        use std::iter::once;

        nums.iter()
            .filter(|x| **x % 2 == 0)
            .zip(nums.iter().filter(|x| **x % 2 != 0))
            .flat_map(|(e, o)| once(e).chain(once(o)))
            .copied()
            .collect()
    }

    // https://leetcode.com/problems/sort-array-by-parity-ii/discuss/1491089/Rust-solution
    pub fn sort_array_by_parity_ii_fp2(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .filter(|&num| num % 2 == 0)
            .zip(nums.iter().filter(|&num| num % 2 != 0))
            .flat_map(|(&even, &odd)| vec![even, odd].into_iter())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![2, 7, 4, 5],
            Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![2, 3], Solution::sort_array_by_parity_ii(vec![2, 3]));
    }
}
