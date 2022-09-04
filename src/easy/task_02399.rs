// 2399. Check Distances Between Same Letters
// https://leetcode.com/problems/check-distances-between-same-letters/

use crate::Solution;

impl Solution {
    pub fn check_distances(s: String, mut distance: Vec<i32>) -> bool {
        for (b, pos) in s.bytes().zip(0..) {
            let dist = &mut distance[(b - b'a') as usize];
            match *dist >= 0 {
                true => *dist = -(*dist + pos + 1), // set next pos with negative flag
                false if *dist != -pos => return false, // check wrong distance
                _ => (),
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::check_distances(
            "abaccb".to_string(),
            vec![1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::check_distances(
            "aa".to_string(),
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ));
    }
}
