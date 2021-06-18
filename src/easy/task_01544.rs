// 1544. Make The String Great
// https://leetcode.com/problems/make-the-string-great/

use crate::Solution;

impl Solution {
    pub fn make_good(s: String) -> String {
        let diff_cases =
            |c1: char, c2: char| c1.to_ascii_lowercase() == c2.to_ascii_lowercase() && c1 != c2;
        s.chars()
            .into_iter()
            .fold(Vec::with_capacity(s.len()), |mut v: Vec<char>, c| {
                match v.last() {
                    Some(prev_c) if diff_cases(c, *prev_c) => {
                        v.pop();
                    }
                    _ => v.push(c),
                }
                v
            })
            .iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "leetcode".to_owned(),
            Solution::make_good("leEeetcode".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!("".to_owned(), Solution::make_good("abBAcC".to_owned()));
    }
}
