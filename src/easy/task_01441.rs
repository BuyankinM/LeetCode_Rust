// 1441. Build an Array With Stack Operations
// https://leetcode.com/problems/build-an-array-with-stack-operations/

use crate::Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, _n: i32) -> Vec<String> {
        let mut res = Vec::with_capacity(target.len());
        let mut i = 0;

        for m in 1..=*target.last().unwrap() {
            res.push("Push".to_string());
            match target[i] == m {
                true => i += 1,
                false => res.push("Pop".to_string()),
            };
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![
                "Push".to_string(),
                "Push".to_string(),
                "Pop".to_string(),
                "Push".to_string()
            ],
            Solution::build_array(vec![1, 3], 3)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec!["Push".to_string(), "Push".to_string(), "Push".to_string()],
            Solution::build_array(vec![1, 2, 3], 3)
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            vec!["Push".to_string(), "Push".to_string()],
            Solution::build_array(vec![1, 2], 4)
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            vec![
                "Push".to_string(),
                "Pop".to_string(),
                "Push".to_string(),
                "Push".to_string(),
                "Push".to_string()
            ],
            Solution::build_array(vec![2, 3, 4], 4)
        );
    }
}
