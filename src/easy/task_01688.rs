// 1688. Count of Matches in Tournament
// https://leetcode.com/problems/count-of-matches-in-tournament/

use crate::Solution;

impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        n - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::number_of_matches(7));
    }

    #[test]
    fn test_2() {
        assert_eq!(13, Solution::number_of_matches(14));
    }
}
