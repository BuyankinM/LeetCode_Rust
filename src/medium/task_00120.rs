// 120. Triangle
// https://leetcode.com/problems/triangle/

use crate::Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.len() == 1 {
            return triangle[0][0];
        }

        let mut dp = triangle.last().unwrap().clone();
        for row in triangle.iter().rev().skip(1) {
            for (i, &num) in row.iter().enumerate() {
                dp[i] = num + dp[i].min(dp[i + 1]);
            }
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            11,
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(-10, Solution::minimum_total(vec![vec![-10]]));
    }
}
