// 1598. Crawler Log Folder
// https://leetcode.com/problems/crawler-log-folder/

use crate::Solution;

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        logs.iter().fold(0, |depth, s| {
            if s.starts_with("..") {
                (depth - 1).max(0)
            } else if s.starts_with('.') {
                depth
            } else {
                depth + 1
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::min_operations(vec![
                "d1/".to_owned(),
                "d2/".to_owned(),
                "../".to_owned(),
                "d21/".to_owned(),
                "./".to_owned()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::min_operations(vec![
                "d1/".to_owned(),
                "d2/".to_owned(),
                "./".to_owned(),
                "d3/".to_owned(),
                "../".to_owned(),
                "d31/".to_owned()
            ])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            0,
            Solution::min_operations(vec![
                "d1/".to_owned(),
                "../".to_owned(),
                "../".to_owned(),
                "../".to_owned()
            ])
        );
    }
}
