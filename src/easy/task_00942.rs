// 942. DI String Match
// https://leetcode.com/problems/di-string-match/

use crate::Solution;

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut res = Vec::with_capacity(s.len() + 1);
        let (mut min_val, mut max_val) = (0, s.len() as i32);

        s.as_bytes().iter().for_each(|b| match *b {
            b'I' => {
                res.push(min_val);
                min_val += 1
            }
            _ => {
                res.push(max_val);
                max_val -= 1
            }
        });
        res.push(max_val);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![0, 4, 1, 3, 2],
            Solution::di_string_match("IDID".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![0, 1, 2, 3],
            Solution::di_string_match("III".to_string())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![3, 2, 0, 1],
            Solution::di_string_match("DDI".to_string())
        );
    }
}
