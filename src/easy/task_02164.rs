// 2164. Sort Even and Odd Indices Independently
// https://leetcode.com/problems/sort-even-and-odd-indices-independently/

use crate::Solution;

impl Solution {
    pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 3 {
            return nums;
        }

        let mut evens = nums.iter().step_by(2).collect::<Vec<_>>();
        let mut odds = nums[1..].iter().step_by(2).collect::<Vec<_>>();
        evens.sort_unstable();
        odds.sort_unstable_by_key(|&&x| -x);

        let (mut evens_it, mut odds_it) = (evens.into_iter(), odds.into_iter());
        let mut result = Vec::new();
        let mut any = 1;
        while any != 0 {
            any = 0;

            if let Some(&even) = evens_it.next() {
                result.push(even);
                any |= 1;
            }

            if let Some(&odd) = odds_it.next() {
                result.push(odd);
                any |= 2;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![2, 3, 4, 1], Solution::sort_even_odd(vec![4, 1, 2, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![2, 1], Solution::sort_even_odd(vec![2, 1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![2, 3, 4, 1, 6],
            Solution::sort_even_odd(vec![4, 1, 2, 3, 6])
        );
    }
}
