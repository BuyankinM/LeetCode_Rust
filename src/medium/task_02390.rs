// 2390. Removing Stars From a String
// https://leetcode.com/problems/removing-stars-from-a-string/

use crate::Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        s.chars()
            .fold(String::with_capacity(s.len()), |mut res, c| {
                match c != '*' {
                    true => res.push(c),
                    false => {
                        res.pop();
                    }
                };
                res
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::remove_stars("leet**cod*e".to_string()),
            "lecoe".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::remove_stars("erase*****".to_string()),
            "".to_string()
        );
    }
}
