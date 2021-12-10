// 747. Largest Number At Least Twice of Others
// https://leetcode.com/problems/largest-number-at-least-twice-of-others/

use crate::Solution;

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut ind = -1;
        let (mut max, mut prev_max) = (0, 0);

        nums.iter().enumerate().for_each(|(i, &x)| {
            if x > max {
                prev_max = std::mem::replace(&mut max, x);
                ind = i as i32;
            } else if x > prev_max {
                prev_max = x;
            }
        });

        match max >= prev_max * 2 {
            true => ind,
            false => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::dominant_index(vec![3, 6, 1, 0]));
    }

    #[test]
    fn test_2() {
        assert_eq!(-1, Solution::dominant_index(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::dominant_index(vec![1]));
    }

    #[test]
    fn test_4() {
        assert_eq!(3, Solution::dominant_index(vec![0, 0, 0, 1]));
    }
}
