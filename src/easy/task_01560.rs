// 1560. Most Visited Sector in a Circular Track
// https://leetcode.com/problems/most-visited-sector-in-a-circular-track/

use crate::Solution;

impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let &start = rounds.first().unwrap();
        let &end = rounds.last().unwrap();
        match end >= start {
            true => (start..=end).collect(),
            false => (1..=end).chain(start..=n).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![1, 2], Solution::most_visited(4, vec![1, 3, 1, 2]));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![2],
            Solution::most_visited(2, vec![2, 1, 2, 1, 2, 1, 2, 1, 2])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7],
            Solution::most_visited(7, vec![1, 3, 5, 7])
        );
    }
}
