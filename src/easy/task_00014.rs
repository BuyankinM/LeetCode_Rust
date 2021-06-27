// 14. Longest Common Prefix
// https://leetcode.com/problems/longest-common-prefix/

use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let first = &strs[0];
        let mut max_ind = first.len();

        for next in strs.iter().skip(1) {
            if let Some(((last_ind, _), _)) = first[..max_ind]
                .char_indices()
                .zip(next.chars())
                .take_while(|((_, c1), c2)| c1 == c2)
                .last()
            {
                max_ind = last_ind + 1;
            } else {
                return "".to_owned();
            }
        }
        first[..max_ind].to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "fl".to_owned(),
            Solution::longest_common_prefix(vec![
                "flower".to_owned(),
                "flow".to_owned(),
                "flight".to_owned()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "".to_owned(),
            Solution::longest_common_prefix(vec![
                "dog".to_owned(),
                "racecar".to_owned(),
                "car".to_owned()
            ])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "".to_owned(),
            Solution::longest_common_prefix(vec!["".to_owned()])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            "a".to_owned(),
            Solution::longest_common_prefix(vec!["ab".to_owned(), "a".to_owned()])
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            "".to_owned(),
            Solution::longest_common_prefix(vec![
                "reflower".to_owned(),
                "flow".to_owned(),
                "flight".to_owned()
            ])
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            "c".to_owned(),
            Solution::longest_common_prefix(vec!["cir".to_owned(), "car".to_owned()])
        );
    }
}
