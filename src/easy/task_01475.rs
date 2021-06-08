// 1475. Final Prices With a Special Discount in a Shop
// https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/

use crate::Solution;

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(prices.len());

        'main: for (i, &ai) in prices.iter().enumerate() {
            for &aj in prices.iter().skip(i + 1) {
                if aj <= ai {
                    res.push(ai - aj);
                    continue 'main;
                }
            }
            res.push(ai);
        }
        res
    }

    pub fn final_prices_simple(mut prices: Vec<i32>) -> Vec<i32> {
        for i in 0..prices.len() - 1 {
            let ai = prices[i];
            for j in i + 1..prices.len() {
                if prices[j] <= ai {
                    prices[i] = ai - prices[j];
                    break;
                }
            }
        }
        prices
    }

    pub fn final_prices_stack(mut prices: Vec<i32>) -> Vec<i32> {
        let l = prices.len();
        let mut stack = Vec::with_capacity(l);

        for i in 0..l {
            while !stack.is_empty() && prices[*stack.last().unwrap()] >= prices[i] {
                prices[stack.pop().unwrap()] -= prices[i];
            }
            stack.push(i);
        }
        prices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![4,2,4,2,3], Solution::final_prices(vec![8,4,6,2,3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![1,2,3,4,5], Solution::final_prices_simple(vec![1,2,3,4,5]));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![9,0,1,6], Solution::final_prices_stack(vec![10,1,1,6]));
    }
}
