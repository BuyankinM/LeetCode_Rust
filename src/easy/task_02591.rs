// 2591. Distribute Money to Maximum Children
// https://leetcode.com/problems/distribute-money-to-maximum-children/

use crate::Solution;

impl Solution {
    pub fn dist_money(money: i32, children: i32) -> i32 {
        match (money / 8, money % 8) {
            _ if money < children => -1,
            (n, 4) if children == n + 1 => n - 1,
            (n, m) if (n > children) || (n == children && m > 0) => children - 1,
            (n, m) if n == 0 || (children - n) <= m => n,
            _ => (money - children) / 7,
        }
    }

    // https://leetcode.com/problems/distribute-money-to-maximum-children/solutions/3312185/3-lines-math-based-solution-o-1-rust/
    pub fn dist_money_opt(money: i32, children: i32) -> i32 {
        if money < children {
            return -1;
        };
        // (1)             x + 8 * y = money
        // (2)             x + y = children
        // From (2)        x = children - y
        // From (1, 2)     children - y + 8 * y = money
        // Then            7 * y = money - children
        // Since the answer is less or equal to children, then y is:
        let mut y = children.min((money - children) / 7);

        // Ensure that will not be any children left with only 4$.
        while money - 8 * y == 4 && children - y == 1 ||
              // or any money left without being distributed.
              money - 8 * y > 0 && children - y == 0
        {
            y -= 1
        }
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::dist_money(20, 3), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::dist_money(16, 2), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::dist_money(24, 3), 3);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::dist_money(2, 2), 0);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::dist_money(1, 2), -1);
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::dist_money(4, 2), 0);
    }

    #[test]
    fn test_7() {
        assert_eq!(Solution::dist_money(8, 2), 0);
    }

    #[test]
    fn test_8() {
        assert_eq!(Solution::dist_money(17, 2), 1);
    }

    #[test]
    fn test_9() {
        assert_eq!(Solution::dist_money(9, 2), 1);
    }

    #[test]
    fn test_10() {
        assert_eq!(Solution::dist_money(24, 2), 1);
    }

    #[test]
    fn test_11() {
        assert_eq!(Solution::dist_money(16, 10), 0);
    }

    #[test]
    fn test_12() {
        assert_eq!(Solution::dist_money(24, 18), 0);
    }

    #[test]
    fn test_13() {
        assert_eq!(Solution::dist_money(32, 19), 1);
    }

    #[test]
    fn test_14() {
        assert_eq!(Solution::dist_money(16, 9), 1);
    }
}
