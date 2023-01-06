// 1833. Maximum Ice Cream Bars
// https://leetcode.com/problems/maximum-ice-cream-bars/description/

use crate::Solution;

impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        let mut res = 0;
        costs.sort_unstable();

        for &cost in &costs {
            match cost <= coins {
                true => coins -= cost,
                false => break,
            }
            res += 1;
        }
        res
    }

    // https://leetcode.com/problems/maximum-ice-cream-bars/solutions/3007031/rust-functional-style-with-comments/
    pub fn max_ice_cream_func(mut costs: Vec<i32>, coins: i32) -> i32 {
        costs.sort_unstable();
        costs
            .into_iter()
            .scan(coins, |left, cost| {
                *left -= cost;
                (*left >= 0).then_some(())
            })
            .count() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::max_ice_cream(vec![1, 3, 2, 4, 1], 7));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::max_ice_cream(vec![10, 6, 8, 7, 7, 8], 5));
    }

    #[test]
    fn test_3() {
        assert_eq!(6, Solution::max_ice_cream(vec![1, 6, 3, 1, 2, 5], 20));
    }
}
