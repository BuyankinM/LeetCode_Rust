// 2549. Count Distinct Numbers on Board
// https://leetcode.com/problems/count-distinct-numbers-on-board/description/

use crate::Solution;

impl Solution {
    pub fn distinct_integers(n: i32) -> i32 {
        match n >= 3 {
            true => n - 1,
            false => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::distinct_integers(3));
    }

    #[test]
    fn test_2() {
        assert_eq!(98, Solution::distinct_integers(99));
    }
    #[test]
    fn test_3() {
        assert_eq!(1, Solution::distinct_integers(1));
    }
}
