// 1974. Minimum Time to Type Word Using Special Typewriter
// https://leetcode.com/problems/minimum-time-to-type-word-using-special-typewriter/

use crate::Solution;

impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        word.as_bytes()
            .iter()
            .fold((word.len() as i32, b'a'), |(sum, prev), &b| {
                let steps = (prev as i32 - b as i32).abs();
                (sum + steps.min(26 - steps), b)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::min_time_to_type("abc".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(7, Solution::min_time_to_type("bza".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(34, Solution::min_time_to_type("zjpc".to_string()));
    }
}
