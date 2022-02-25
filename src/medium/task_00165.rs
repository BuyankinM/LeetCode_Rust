// 165. Compare Version Numbers
// https://leetcode.com/problems/compare-version-numbers/

use crate::Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        use std::cmp::Ordering::{Greater, Less};

        let (mut it_1, mut it_2) = (version1.split('.'), version2.split('.'));
        loop {
            let (s1, s2) = (it_1.next(), it_2.next());
            if s1.is_none() && s2.is_none() {
                break;
            }

            let s1 = s1.unwrap_or("0").trim_start_matches('0');
            let s2 = s2.unwrap_or("0").trim_start_matches('0');
            if s1.len() != s2.len() {
                return (s1.len() as i32 - s2.len() as i32).signum();
            }

            for (c1, c2) in s1.chars().zip(s2.chars()) {
                match c1.cmp(&c2) {
                    Greater => return 1,
                    Less => return -1,
                    _ => (),
                }
            }
        }
        0
    }

    // https://leetcode.com/problems/compare-version-numbers/discuss/495008/Rust-simple-solution
    pub fn compare_version_vec(version1: String, version2: String) -> i32 {
        let mut v1: Vec<u32> = version1.split('.').map(|s| s.parse().unwrap()).collect();
        let mut v2: Vec<u32> = version2.split('.').map(|s| s.parse().unwrap()).collect();
        while v1.len() < v2.len() {
            v1.push(0);
        }
        while v2.len() < v1.len() {
            v2.push(0);
        }
        match v1.cmp(&v2) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Greater => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            0,
            Solution::compare_version("1.01".to_string(), "1.001".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            0,
            Solution::compare_version("1.0".to_string(), "1.0.0".to_string())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            -1,
            Solution::compare_version("0.1".to_string(), "1.1".to_string())
        );
    }
}
