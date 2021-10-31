// 2042. Check if Numbers Are Ascending in a Sentence
// https://leetcode.com/problems/check-if-numbers-are-ascending-in-a-sentence/

use crate::Solution;

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut num = 0;
        for n in s.split(' ').filter_map(|ss| ss.parse::<i32>().ok()) {
            match n > num {
                true => num = n,
                false => return false,
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
        assert!(Solution::are_numbers_ascending(
            "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string()
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::are_numbers_ascending(
            "hello world 5 x 5".to_string()
        ));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::are_numbers_ascending(
            "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s".to_string()
        ));
    }

    #[test]
    fn test_4() {
        assert!(Solution::are_numbers_ascending("4 5 11 26".to_string()));
    }
}
