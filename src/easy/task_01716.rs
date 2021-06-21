// 1716. Calculate Money in Leetcode Bank
// https://leetcode.com/problems/calculate-money-in-leetcode-bank/

use crate::Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut s = 0;
        for i in 1..=n {
            s += if i % 7 > 0 {
                (i % 7) + (i / 7)
            } else {
                6 + (i / 7)
            }
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(10, Solution::total_money(4));
    }

    #[test]
    fn test_2() {
        assert_eq!(37, Solution::total_money(10));
    }

    #[test]
    fn test_3() {
        assert_eq!(96, Solution::total_money(20));
    }
}
