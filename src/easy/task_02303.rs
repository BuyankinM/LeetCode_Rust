// 2303. Calculate Amount Paid in Taxes
// https://leetcode.com/problems/calculate-amount-paid-in-taxes/

use crate::Solution;

impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, mut income: i32) -> f64 {
        let mut total = 0.0;
        let mut prev_bound = 0;
        for pair in &brackets {
            if income == 0 {
                break;
            }

            let (upper_bound, percent) = (pair[0], pair[1]);
            let sum = income.min(upper_bound - prev_bound);
            total += (sum * percent) as f64 / 100.0;
            income -= sum;
            prev_bound = upper_bound;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2.65,
            Solution::calculate_tax(vec![vec![3, 50], vec![7, 10], vec![12, 25]], 10)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            0.25,
            Solution::calculate_tax(vec![vec![1, 0], vec![4, 25], vec![5, 50]], 2)
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(0.0, Solution::calculate_tax(vec![vec![2, 50]], 0));
    }
}
