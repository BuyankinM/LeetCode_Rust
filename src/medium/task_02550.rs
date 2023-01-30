// 2550. Count Collisions of Monkeys on a Polygon
// https://leetcode.com/problems/count-collisions-of-monkeys-on-a-polygon/

use crate::Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn monkey_move(mut n: i32) -> i32 {
        let (mut res, mut base) = (1, 2);
        while n > 0 {
            if n % 2 == 1 {
                res = (res * base) % MOD;
            }
            base = (base * base) % MOD;
            n /= 2;
        }
        ((res - 2 + MOD) % MOD) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::monkey_move(3));
    }

    #[test]
    fn test_2() {
        assert_eq!(14, Solution::monkey_move(4));
    }

    #[test]
    fn test_3() {
        assert_eq!(688423208, Solution::monkey_move(1000));
    }

    #[test]
    fn test_4() {
        assert_eq!(124999999, Solution::monkey_move(500_000_000));
    }
}
