// 70. Climbing Stairs
// https://leetcode.com/problems/climbing-stairs/

use crate::Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (0..n)
            .fold((1, 0), |(res, prev), _| (res + prev, res))
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::climb_stairs(1));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::climb_stairs(2));
    }

    #[test]
    fn test_3() {
        assert_eq!(8, Solution::climb_stairs(5));
    }
}
