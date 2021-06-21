// 1704. Determine if String Halves Are Alike
// https://leetcode.com/problems/determine-if-string-halves-are-alike/

use crate::Solution;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        use std::collections::HashSet;
        let vowels: HashSet<char> = "aeiouAEIOU".chars().collect();
        let mut counter = 0;
        let mid = s.len() / 2;
        for (ind, c) in s.chars().enumerate() {
            if vowels.contains(&c) {
                if ind < mid {
                    counter += 1;
                } else {
                    counter -= 1;
                }
            }
        }

        counter == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::halves_are_alike("book".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(false, Solution::halves_are_alike("textbook".to_owned()));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            false,
            Solution::halves_are_alike("MerryChristmas".to_owned())
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(true, Solution::halves_are_alike("AbCdEfGh".to_owned()));
    }
}
