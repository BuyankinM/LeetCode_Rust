// 714. Best Time to Buy and Sell Stock with Transaction Fee
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/
use crate::Solution;
use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut cash = 0;
        let mut hold = -prices[0];

        println!(
            "price = {},  cash = {:2},  hold = {:2}",
            prices[0], cash, hold
        );

        for price in prices.iter().skip(1) {
            cash = max(cash, hold + price - fee);
            hold = max(hold, cash - price);
            println!("price = {},  cash = {:2},  hold = {:2}", price, cash, hold);
        }

        cash
    }
}
