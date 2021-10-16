// 1071. Greatest Common Divisor of Strings
// https://leetcode.com/problems/greatest-common-divisor-of-strings/

use crate::Solution;

impl Solution {
    fn gcd(a: usize, b: usize) -> usize {
        match b > 0 {
            true => Solution::gcd(b, a % b),
            false => a,
        }
    }

    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let (sb1, sb2) = (str1.as_bytes(), str2.as_bytes());
        let (l1, l2) = (str1.len(), str2.len());
        let l = Solution::gcd(l1, l2);

        if sb1.iter().enumerate().all(|(i, &b1)| b1 == sb2[i % l])
            && sb2.iter().enumerate().all(|(i, &b2)| b2 == sb1[i % l])
        {
            String::from_utf8(sb1[..l].to_vec()).unwrap()
        } else {
            "".to_string()
        }
    }

    pub fn gcd_of_strings_short(str1: String, str2: String) -> String {
        match str1.chars().chain(str2.chars()).eq(str2.chars().chain(str1.chars())) {
            true => str1
                .chars()
                .take(Solution::gcd(str1.len(), str2.len()))
                .collect(),
            false => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "ABC".to_string(),
            Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "AB".to_string(),
            Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "".to_string(),
            Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string())
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            "".to_string(),
            Solution::gcd_of_strings("ABCDEF".to_string(), "ABC".to_string())
        );
    }
}
