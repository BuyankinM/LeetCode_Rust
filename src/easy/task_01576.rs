// 1576. Replace All ?'s to Avoid Consecutive Repeating Characters
// https://leetcode.com/problems/replace-all-s-to-avoid-consecutive-repeating-characters/

use crate::Solution;

impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut prev = 0;
        let sb = s.as_bytes();
        let res = sb
            .iter()
            .enumerate()
            .map(|(ind, &b)| match b {
                b'?' => {
                    let mut new_b = (prev + 1) % 26;
                    if (ind + 1) < sb.len() && (new_b + b'a') == sb[ind + 1] {
                        new_b = (sb[ind + 1] - b'a' + 1) % 26;
                    }
                    prev = new_b;
                    new_b + b'a'
                }
                _ => {
                    prev = b;
                    b
                }
            })
            .collect();
        String::from_utf8(res).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("bzs".to_owned(), Solution::modify_string("?zs".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "ubvpw".to_owned(),
            Solution::modify_string("ubv?w".to_owned())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "jdqgacb".to_owned(),
            Solution::modify_string("j?qg??b".to_owned())
        );
    }
}
