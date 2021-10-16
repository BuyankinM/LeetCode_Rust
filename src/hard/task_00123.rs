// 123. Best Time to Buy and Sell Stock III
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/

use crate::Solution;

impl Solution {
    pub fn max_profit_3(prices: Vec<i32>) -> i32 {
        let (mut buy_1tr, mut buy_2tr) = (std::i32::MAX, std::i32::MAX);
        let (mut sell_1tr, mut sell_2tr) = (0, 0);

        prices.iter().for_each(|&price| {
            buy_1tr = buy_1tr.min(price);
            sell_1tr = sell_1tr.max(price - buy_1tr);
            buy_2tr = buy_2tr.min(price - sell_1tr);
            sell_2tr = sell_2tr.max(price - buy_2tr);
        });

        sell_2tr
    }

    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/discuss/1523239/Rust11ms-beat-100-with-explainations
    pub fn max_profit_3_fast(prices: Vec<i32>) -> i32 {
        let mut first_buy_price = prices[0];
        let mut first_transaction_profit = 0;
        let mut total_buy_cost = first_buy_price;
        let mut total_profit = 0;

        // we want to max two profit variable while keep buy price/cost to minimum
        for &price in &prices[1..] {
            // max first transaction profit
            let current_first_buy_price = first_buy_price.min(price);
            let current_first_transaction_profit =
                first_transaction_profit.max(price - first_buy_price);

            // max total transaction profit
            // as max total_profit=max second price - min second buy price + (max first buy price - min first buy price) before max second price occurred
            // so we need to subtract first_transaction_profit from second buy price to record first_transaction_profit
            // which is max total_profit=max second price - (min second buy price - (max first buy price - min first buy price) before max second price occurred)
            let current_total_buy_cost = total_buy_cost.min(price - first_transaction_profit);
            let current_total_profit = total_profit.max(price - total_buy_cost);

            // update
            first_buy_price = current_first_buy_price;
            first_transaction_profit = current_first_transaction_profit;
            total_profit = current_total_profit;
            total_buy_cost = current_total_buy_cost;
        }

        total_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::max_profit_3(vec![3, 3, 5, 0, 0, 3, 1, 4]));
    }

    #[test]
    fn test_2() {
        assert_eq!(6, Solution::max_profit_3_fast(vec![3, 3, 5, 0, 0, 3, 1, 4]));
    }

    #[test]
    fn test_3() {
        assert_eq!(4, Solution::max_profit_3(vec![1, 2, 3, 4, 5]));
    }
}
