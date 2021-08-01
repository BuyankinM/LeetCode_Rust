// 1592. Rearrange Spaces Between Words
// https://leetcode.com/problems/rearrange-spaces-between-words/

use crate::Solution;

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let count_whitespace = text.chars().filter(|x| *x == ' ').count();
        if count_whitespace == 0 {
            return text;
        }
        let div_mod = |x, y| (x / y, x % y);
        let words = text.split_whitespace().collect::<Vec<_>>();

        if words.len() == 1 {
            words[0].to_owned() + " ".repeat(count_whitespace).as_str()
        } else {
            let (div_sep, mod_sep) = div_mod(count_whitespace, words.len() - 1);
            words.join(" ".repeat(div_sep).as_str()) + " ".repeat(mod_sep).as_str()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "this   is   a   sentence".to_owned(),
            Solution::reorder_spaces("  this   is  a sentence ".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "practice   makes   perfect ".to_owned(),
            Solution::reorder_spaces(" practice   makes   perfect".to_owned())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "hello   world".to_owned(),
            Solution::reorder_spaces("hello   world".to_owned())
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            "walks  udp  package  into  bar  a ".to_owned(),
            Solution::reorder_spaces("  walks  udp package   into  bar a".to_owned())
        );
    }
}
