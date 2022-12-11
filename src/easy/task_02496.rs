// 2496. Maximum Value of a String in an Array
// https://leetcode.com/problems/maximum-value-of-a-string-in-an-array/

use crate::Solution;

impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        strs.iter().fold(0, |max_val, s| {
            max_val.max(s.parse::<i32>().unwrap_or(s.len() as i32))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            5,
            Solution::maximum_value(vec![
                "alic3".to_string(),
                "bob".to_string(),
                "3".to_string(),
                "4".to_string(),
                "00000".to_string()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            1,
            Solution::maximum_value(vec![
                "1".to_string(),
                "01".to_string(),
                "001".to_string(),
                "0001".to_string()
            ])
        );
    }
}
