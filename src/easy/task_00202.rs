// 202. Happy Number
// https://leetcode.com/problems/happy-number/

use crate::Solution;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        loop {
            let mut s = 0;
            while n > 0 {
                s += (n % 10).pow(2);
                n /= 10;
            }
            match s {
                1 | 4 => break s == 1,
                _ => n = s,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_happy(19));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_happy(2));
    }

    #[test]
    fn test_3() {
        assert!(Solution::is_happy(1111111));
    }
}
