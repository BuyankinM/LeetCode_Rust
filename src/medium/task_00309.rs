// 309. Best Time to Buy and Sell Stock with Cooldown
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/

use crate::Solution;

impl Solution {
    pub fn max_profit_new(prices: Vec<i32>) -> i32 {
        let l = prices.len();
        if l == 1 {
            return 0;
        }

        let (mut buy, mut sell) = (vec![0; l], vec![0; l]);
        buy[0] = -prices[0];
        buy[1] = -(prices[0].min(prices[1]));
        sell[1] = 0.max(buy[0] + prices[1]);

        for (cur_price, i) in prices[2..].iter().zip(2..) {
            buy[i] = buy[i - 1].max(sell[i - 2] - cur_price);
            sell[i] = sell[i - 1].max(buy[i - 1] + cur_price);
        }

        sell[l - 1]
    }

    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/discuss/1522047/Rust-solution
    pub fn max_profit_func(prices: Vec<i32>) -> i32 {
        prices
            .into_iter()
            .fold((0, 0, i32::MIN), |(a, b, c), x| {
                (a.max(c + x), b.max(a), c.max(b - x))
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::max_profit_new(vec![1, 2, 3, 0, 2]));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::max_profit_new(vec![1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            601,
            Solution::max_profit_new(vec![1, 2, 3, 0, 2, 100, 23, 400, 500, 600, 5])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            601,
            Solution::max_profit_func(vec![1, 2, 3, 0, 2, 100, 23, 400, 500, 600, 5])
        );
    }
}
