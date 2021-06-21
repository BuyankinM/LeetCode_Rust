// 1678. Goal Parser Interpretation
// https://leetcode.com/problems/goal-parser-interpretation/

use crate::Solution;

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut res = String::new();
        let mut prev = ' ';

        for c in command.chars() {
            match (c, prev) {
                ('G', _) => res.push(c),
                (')', '(') => res.push('o'),
                (')', 'l') => res.push_str("al"),
                _ => prev = c,
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
        assert_eq!("Goal".to_owned(), Solution::interpret("G()(al)".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "Gooooal".to_owned(),
            Solution::interpret("G()()()()(al)".to_owned())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "alGalooG".to_owned(),
            Solution::interpret("(al)G(al)()()G".to_owned())
        );
    }
}
