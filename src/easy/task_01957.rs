// 1957. Delete Characters to Make Fancy String
// https://leetcode.com/problems/delete-characters-to-make-fancy-string/

use crate::Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut res = String::with_capacity(s.len());
        let (mut count, mut prev) = (0, ' ');
        s.chars().for_each(|c| {
            match c == prev {
                true => count += 1,
                false => {
                    count = 1;
                    prev = c;
                }
            };
            if count < 3 {
                res.push(c);
            }
        });
        res
    }

    pub fn make_fancy_string_fold(s: String) -> String {
        let (mut count, mut prev) = (0, ' ');
        s.chars()
            .fold(String::with_capacity(s.len()), |mut res, c| {
                match c == prev {
                    true => count += 1,
                    false => count = 1,
                };
                if count < 3 {
                    res.push(c);
                }
                prev = c;
                res
            })
    }

    pub fn make_fancy_string_bytes(s: String) -> String {
        let mut sb = s.into_bytes();
        let (mut count, mut pos) = (0, 0);

        for i in 0..sb.len() {
            match pos == 0 || sb[i] == sb[pos - 1] {
                true => count += 1,
                false => count = 1,
            };
            if count < 3 {
                if pos < i {
                    sb[pos] = sb[i];
                }
                pos += 1;
            }
        }

        String::from_utf8(sb[..pos].to_vec()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "leetcode".to_string(),
            Solution::make_fancy_string("leeetcode".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "aabaa".to_string(),
            Solution::make_fancy_string_fold("aaabaaaa".to_string())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "aab".to_string(),
            Solution::make_fancy_string_bytes("aab".to_string())
        );
    }
}
