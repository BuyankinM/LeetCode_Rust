// 1742. Maximum Number of Balls in a Box
// https://leetcode.com/problems/maximum-number-of-balls-in-a-box/

use crate::Solution;

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut boxes = [0; 46];
        let sum_of_digits = |mut x: i32| {
            let mut s = 0;
            while x > 0 {
                s += x % 10;
                x /= 10;
            }
            s as usize
        };

        (low_limit..=high_limit).for_each(|val| boxes[sum_of_digits(val)] += 1);
        boxes.iter().fold(0, |max_val, &x| max_val.max(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::count_balls(1, 10));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::count_balls(5, 15));
    }

    #[test]
    fn test_3() {
        assert_eq!(2, Solution::count_balls(19, 28));
    }
}
