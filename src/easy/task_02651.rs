// 2651. Calculate Delayed Arrival Time
// https://leetcode.com/problems/calculate-delayed-arrival-time/

use crate::Solution;

impl Solution {
    pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
        (arrival_time + delayed_time) % 24
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(20, Solution::find_delayed_arrival_time(15, 5));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::find_delayed_arrival_time(13, 11));
    }
}
