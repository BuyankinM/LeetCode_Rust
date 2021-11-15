// 441. Arranging Coins
// https://leetcode.com/problems/arranging-coins/

use crate::Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        // formula for the sum of an arithmetic progression: s = n(n+1) / 2
        // we get the quadratic equation: n^2 + n - 2s = 0
        // discriminant: D = b^2 - 4ac = 1 + 8s
        // we need positive root of the equation: n = (-b + sqrt(D)) / 2a = (sqrt(8s+1) - 1) / 2
        // last operation is floor to get integer number of steps
        (((8.0 * n as f64 + 1.0).sqrt() - 1.0) / 2.0).floor() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::arrange_coins(5));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::arrange_coins(8));
    }

    #[test]
    fn test_3() {
        assert_eq!(9, Solution::arrange_coins(45));
    }
}
