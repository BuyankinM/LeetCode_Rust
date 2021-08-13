// 1952. Three Divisors
// https://leetcode.com/problems/three-divisors/

use crate::Solution;

impl Solution {
    pub fn is_three(n: i32) -> bool {
        match n {
            4 => return true,
            v if v < 4 || v % 2 == 0 => return false,
            _ => (),
        }

        let i32_sqrt = |x| -> i32 { (x as f32).sqrt() as i32 };
        let sq = i32_sqrt(n);

        // test for square root
        if sq * sq != n {
            return false;
        }

        // test for simple
        for i in (3..i32_sqrt(sq) + 1).step_by(2) {
            if sq % i == 0 {
                return false;
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
        assert!(!Solution::is_three(3));
    }

    #[test]
    fn test_2() {
        assert!(Solution::is_three(4));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::is_three(16));
    }

    #[test]
    fn test_4() {
        assert!(Solution::is_three(5329));
    }
}
