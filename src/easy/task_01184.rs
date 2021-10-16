// 1184. Distance Between Bus Stops
// https://leetcode.com/problems/distance-between-bus-stops/

use crate::Solution;

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let (s0, s1) = match start <= destination {
            true => (start as usize, destination as usize),
            false => (destination as usize, start as usize),
        };

        distance[s0..s1]
            .iter()
            .sum::<i32>()
            .min(distance[..s0].iter().sum::<i32>() + distance[s1..].iter().sum::<i32>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            1,
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 1)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 2)
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            4,
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 3)
        );
    }
}
