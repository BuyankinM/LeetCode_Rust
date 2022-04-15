// 409. Longest Palindrome
// https://leetcode.com/problems/longest-palindrome/

use crate::Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut map = std::collections::HashMap::new();
        s.chars().for_each(|c| {
            *map.entry(c).or_insert(0) += 1;
        });

        let mut count = map.values().fold(0, |acc, &v| {
            acc + match v % 2 == 0 {
                true => v,
                false => v - 1,
            }
        });

        if count < s.len() {
            count += 1;
        }

        count as _
    }

    // https://leetcode.com/problems/longest-palindrome/
    pub fn longest_palindrome_short(s: String) -> i32 {
        let mut d: [i32; 128] = [0; 128];
        for &b in s.as_bytes() {
            d[b as usize] += 1;
        }
        let mut answer = 0;
        for n in d.iter() {
            answer += n;
            if n % 2 != 0 {
                answer -= 1;
                if answer % 2 == 0 {
                    answer += 1;
                }
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::longest_palindrome("abccccdd".to_string()), 7);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::longest_palindrome("abccccddrrrtttggghhvb".to_string()),
            17
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::longest_palindrome("a".to_string()), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::longest_palindrome("".to_string()), 0);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::longest_palindrome("aab".to_string()), 3);
    }
}
