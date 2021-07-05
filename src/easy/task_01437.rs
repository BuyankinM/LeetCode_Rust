// 1437. Check If All 1's Are at Least Length K Places Away
// https://leetcode.com/problems/check-if-all-1s-are-at-least-length-k-places-away/

use crate::Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut prev_ind = -1;
        for (ind, _) in nums.iter().enumerate().filter(|(_, x)| **x == 1) {
            match prev_ind != -1 && (ind as i32 - prev_ind - 1) < k {
                true => return false,
                false => prev_ind = ind as i32,
            };
        }
        true
    }

    pub fn k_length_apart_try_fold(nums: Vec<i32>, k: i32) -> bool {
        nums.into_iter()
            .try_fold(0, |rem, x| match x {
                1 if rem > 0 => None,
                1 => Some(k),
                _ => Some(rem - 1),
            })
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            true,
            Solution::k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(false, Solution::k_length_apart(vec![1, 0, 0, 1, 0, 1], 2));
    }

    #[test]
    fn test_3() {
        assert_eq!(true, Solution::k_length_apart(vec![1, 1, 1, 1, 1], 0));
    }

    #[test]
    fn test_4() {
        assert_eq!(true, Solution::k_length_apart(vec![0, 1, 0, 1], 1));
    }

    #[test]
    fn test_5() {
        assert_eq!(
            true,
            Solution::k_length_apart(vec![0, 1, 0, 0, 1, 0, 0, 1], 2)
        );
    }
}
