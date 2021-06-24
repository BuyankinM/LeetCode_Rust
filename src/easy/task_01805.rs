// 1805. Number of Different Integers in a String
// https://leetcode.com/problems/number-of-different-integers-in-a-string/

use crate::Solution;

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        use std::collections::HashSet;

        (&word)
            .split(char::is_alphabetic)
            .filter(|s| !s.is_empty())
            .map(|x| x.trim_start_matches('0'))
            .collect::<HashSet<&str>>()
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::num_different_integers("a123bc34d8ef34".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            2,
            Solution::num_different_integers("leet1234code234".to_owned())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::num_different_integers("a1b01c001".to_owned()));
    }
}
