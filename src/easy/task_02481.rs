// 2481. Minimum Cuts to Divide a Circle
// https://leetcode.com/problems/minimum-cuts-to-divide-a-circle/

use crate::Solution;

impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        match n % 2 {
            1 if n == 1 => 0,
            1 => n,
            _ => n / 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::number_of_cuts(1), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::number_of_cuts(2), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::number_of_cuts(11), 11);
    }
}
