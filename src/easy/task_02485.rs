// 2485. Find the Pivot Integer
// https://leetcode.com/problems/find-the-pivot-integer/

use crate::Solution;

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let total = n * (n + 1) / 2;
        let mut s = 0;
        for x in 1..=n {
            s += x;
            if s == (total - s + x) {
                return x;
            }
        }
        -1
    }

    pub fn pivot_integer_short(n: i32) -> i32 {
        let total = n * (n + 1) / 2;
        let x = (total as f32).sqrt() as i32;
        match x * x == total {
            true => x,
            false => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::pivot_integer(8), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::pivot_integer(1), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::pivot_integer(4), -1);
    }
}
