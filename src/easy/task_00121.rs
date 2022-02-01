// 121. Best Time to Buy and Sell Stock
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

use crate::Solution;

impl Solution {
    pub fn max_profit_task_121(prices: Vec<i32>) -> i32 {
        let (mut max_sell, mut min_buy) = (prices[0], prices[0]);
        let mut profit = 0;
        prices.iter().for_each(|&x| {
            if x < min_buy {
                min_buy = x;
                max_sell = x;
            } else if x > max_sell {
                let new_profit = x - min_buy;
                if new_profit > profit {
                    max_sell = x;
                    profit = new_profit;
                }
            }
        });
        profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::max_profit_task_121(vec![7, 1, 5, 3, 6, 4]));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::max_profit_task_121(vec![7, 6, 4, 3, 1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(4, Solution::max_profit_task_121(vec![3, 2, 6, 5, 0, 3]));
    }
}
