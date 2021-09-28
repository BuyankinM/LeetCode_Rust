// 2011. Final Value of Variable After Performing Operations
// https://leetcode.com/problems/final-value-of-variable-after-performing-operations/

use crate::Solution;

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations
            .iter()
            .flat_map(|s| {
                s.chars().map(|c| match c {
                    '+' => 1,
                    '-' => -1,
                    _ => 0,
                })
            })
            .sum::<i32>()
            / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            1,
            Solution::final_value_after_operations(vec![
                "--X".to_owned(),
                "X++".to_owned(),
                "X++".to_owned()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::final_value_after_operations(vec![
                "++X".to_owned(),
                "++X".to_owned(),
                "X++".to_owned()
            ])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            0,
            Solution::final_value_after_operations(vec![
                "X++".to_owned(),
                "++X".to_owned(),
                "--X".to_owned(),
                "X--".to_owned()
            ])
        );
    }
}
