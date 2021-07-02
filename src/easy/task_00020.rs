// 20. Valid Parentheses
// https://leetcode.com/problems/valid-parentheses/

use crate::Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // we can use ASCII codes
        // () / [] / {} => 40 41 / 91 93 / 123 125
        let mut stack = Vec::new();
        for b in s.into_bytes() {
            match b {
                b'{' | b'(' | b'[' => stack.push(b),
                _ if (b - stack.pop().unwrap_or(0)) <= 2 => (),
                _ => return false,
            };
        }
        stack.is_empty()
    }

    pub fn is_valid_eq(s: String) -> bool {
        let mut stack = Vec::new();
        for i in s.chars() {
            match i {
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '}' | ')' | ']' if Some(i) != stack.pop() => return false,
                _ => (),
            }
        }
        return stack.is_empty();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::is_valid("()".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(true, Solution::is_valid("()[]{}".to_owned()));
    }

    #[test]
    fn test_3() {
        assert_eq!(false, Solution::is_valid("(]".to_owned()));
    }

    #[test]
    fn test_4() {
        assert_eq!(false, Solution::is_valid_eq("([)]".to_owned()));
    }

    #[test]
    fn test_5() {
        assert_eq!(true, Solution::is_valid_eq("{[]}".to_owned()));
    }
}
