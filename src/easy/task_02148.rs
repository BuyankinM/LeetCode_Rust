// 2148. Count Elements With Strictly Smaller and Greater Elements
// https://leetcode.com/problems/count-elements-with-strictly-smaller-and-greater-elements/

use crate::Solution;

impl Solution {
    pub fn count_elements(nums: Vec<i32>) -> i32 {
        let (mut min_val, mut max_val) = (i32::MAX, i32::MIN);
        nums.iter().for_each(|&x| {
            if x < min_val {
                min_val = x
            }
            if x > max_val {
                max_val = x
            }
        });
        nums.iter().filter(|&&x| x > min_val && x < max_val).count() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::count_elements(vec![11, 7, 2, 15]));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::count_elements(vec![-3, 3, 3, 90]));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::count_elements(vec![5, 5, 5]));
    }
}
