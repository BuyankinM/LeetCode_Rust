// 205. Isomorphic Strings
// https://leetcode.com/problems/isomorphic-strings/

use crate::Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let (mut map_st, mut map_ts) = ([0; 256], [0; 256]);
        let s_it = s.as_bytes().iter();
        let t_it = t.as_bytes().iter();
        for (sb, tb) in s_it.zip(t_it).map(|(a, b)| (*a + 1, *b + 1)) {
            let tbc = &mut map_st[sb as usize];
            let sbc = &mut map_ts[tb as usize];
            match *tbc == 0 && *sbc == 0 {
                true => {
                    *tbc = tb;
                    *sbc = sb;
                }
                false if *tbc != tb || *sbc != sb => return false,
                _ => (),
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
        assert!(Solution::is_isomorphic(
            "egg".to_string(),
            "add".to_string()
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_isomorphic(
            "foo".to_string(),
            "bar".to_string()
        ));
    }

    #[test]
    fn test_3() {
        assert!(Solution::is_isomorphic(
            "paper".to_string(),
            "title".to_string()
        ));
    }

    #[test]
    fn test_4() {
        assert!(Solution::is_isomorphic("13".to_string(), "42".to_string()));
    }
}
