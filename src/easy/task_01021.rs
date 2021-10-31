// 1021. Remove Outermost Parentheses
// https://leetcode.com/problems/remove-outermost-parentheses/

use crate::Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut res = String::with_capacity(s.len());
        let mut counter = 0;

        for c in s.chars() {
            let prev_counter = counter;
            counter += if c == '(' { 1 } else { -1 };

            if prev_counter | counter != 1 {
                // not 0 -> 1, 1 -> 0
                res.push(c);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "()()()".to_string(),
            Solution::remove_outer_parentheses("(()())(())".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "()()()()(())".to_string(),
            Solution::remove_outer_parentheses("(()())(())(()(()))".to_string())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "".to_string(),
            Solution::remove_outer_parentheses("()()".to_string())
        );
    }
}
