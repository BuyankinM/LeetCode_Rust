// 1672. Richest Customer Wealth
// https://leetcode.com/problems/richest-customer-wealth/

use crate::Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|v| v.iter().sum()).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            6,
            Solution::maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            10,
            Solution::maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]])
        );
    }
}
