// 2418. Sort the People
// https://leetcode.com/problems/sort-the-people/

use crate::Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut v = names
            .iter()
            .cloned()
            .zip(heights.iter().cloned())
            .collect::<Vec<_>>();
        v.sort_unstable_by_key(|(_, height)| -height);
        v.into_iter().map(|(x, _)| x).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::sort_people(
                vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()],
                vec![180, 165, 170]
            ),
            vec!["Mary".to_string(), "Emma".to_string(), "John".to_string()]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::sort_people(
                vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()],
                vec![155, 185, 150]
            ),
            vec!["Bob".to_string(), "Alice".to_string(), "Bob".to_string()]
        );
    }
}
