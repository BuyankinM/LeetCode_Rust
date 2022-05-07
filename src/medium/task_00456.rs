// 456. 132 Pattern
// https://leetcode.com/problems/132-pattern/

use crate::Solution;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut x3 = i32::MIN;
        let mut stack: Vec<i32> = vec![];
        for &x2 in nums.iter().rev() {
            match x2 < x3 {
                true => return true,
                false => {
                    while !stack.is_empty() && *stack.last().unwrap() < x2 {
                        x3 = stack.pop().unwrap();
                    }
                }
            }
            stack.push(x2);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(!Solution::find132pattern(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_2() {
        assert!(Solution::find132pattern(vec![3, 1, 4, 2]));
    }

    #[test]
    fn test_3() {
        assert!(Solution::find132pattern(vec![-1, 3, 2, 0]));
    }
}
