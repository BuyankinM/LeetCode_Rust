// 746. Min Cost Climbing Stairs
// https://leetcode.com/problems/min-cost-climbing-stairs/

use crate::Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut steps = [0, 0];
        cost.into_iter().enumerate().for_each(|(ind, x)| {
            steps[ind % 2] = steps[0].min(steps[1]) + x;
        });
        *steps.iter().min().unwrap()
    }

    // https://leetcode.com/problems/min-cost-climbing-stairs/discuss/500317/Fun-Two-Line-Rust-Solution/900935
    pub fn min_cost_climbing_stairs_1liner(cost: Vec<i32>) -> i32 {
        cost.iter()
            .chain(&[0])
            .fold((0, 0), |(x, y), t| (y, t + x.min(y)))
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(15, Solution::min_cost_climbing_stairs(vec![10, 15, 20]));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            6,
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::min_cost_climbing_stairs(vec![1, 2]));
    }
}
