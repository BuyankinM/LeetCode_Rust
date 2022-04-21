// 263. Ugly Number
// https://leetcode.com/problems/ugly-number/

use crate::Solution;

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        match n {
            _ if n <= 0 => false,
            mut num => {
                for p in [2, 3, 5] {
                    while num % p == 0 {
                        num /= p;
                    }
                }
                num == 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_ugly(1));
    }

    #[test]
    fn test_2() {
        assert!(Solution::is_ugly(6));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::is_ugly(-6));
    }

    #[test]
    fn test_4() {
        assert!(!Solution::is_ugly(999));
    }
}

