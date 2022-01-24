// 2144. Minimum Cost of Buying Candies With Discount
// https://leetcode.com/problems/minimum-cost-of-buying-candies-with-discount/

use crate::Solution;

impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        let mut total = 0;
        cost.sort_unstable();
        while !cost.is_empty() {
            for i in 0..3 {
                match cost.pop() {
                    Some(val) => {
                        if i < 2 {
                            total += val;
                        }
                    }
                    None => break,
                }
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::minimum_cost(vec![1, 2, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(23, Solution::minimum_cost(vec![6, 5, 7, 9, 2, 2]));
    }

    #[test]
    fn test_3() {
        assert_eq!(10, Solution::minimum_cost(vec![5, 5]));
    }
}
