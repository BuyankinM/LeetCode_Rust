// 392. Is Subsequence
// https://leetcode.com/problems/is-subsequence/

use crate::Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        match s.len() {
            0 => true,
            l if l > t.len() => false,
            l if l == t.len() => s == t,
            _ => {
                let (mut s_it, mut t_it) = (s.chars(), t.chars());
                let (mut s_c, mut t_c) = (s_it.next(), t_it.next());
                while s_c.is_some() && t_c.is_some() {
                    if s_c == t_c {
                        s_c = s_it.next();
                    }
                    t_c = t_it.next();
                }
                s_c.is_none()
            }
        }
    }

    // https://leetcode.com/problems/is-subsequence/discuss/678463/Rust-2-liner
    pub fn is_subsequence_all_any(s: String, t: String) -> bool {
        let mut ti = t.chars();
        s.chars().all(|sc| ti.any(|tc| tc == sc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_subsequence(
            "abc".to_string(),
            "ahbgdc".to_string()
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_subsequence(
            "axc".to_string(),
            "ahbgdc".to_string()
        ));
    }

    #[test]
    fn test_3() {
        assert!(Solution::is_subsequence(
            "".to_string(),
            "ahbgdc".to_string()
        ));
    }

    #[test]
    fn test_4() {
        assert!(!Solution::is_subsequence(
            "aaa".to_string(),
            "aa".to_string(),
        ));
    }
}
