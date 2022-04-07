// 75. Sort Colors
// https://leetcode.com/problems/sort-colors/

use crate::Solution;

impl Solution {
    // https://en.wikipedia.org/wiki/Dutch_national_flag_problem
    pub fn sort_colors(nums: &mut [i32]) {
        let (mut red, mut white, mut blue) = (0, 0, nums.len() - 1);
        while white <= blue {
            match nums[white] {
                0 => {
                    nums.swap(white, red);
                    white += 1;
                    red += 1;
                }
                1 => white += 1,
                2 if blue > 0 => {
                    nums.swap(white, blue);
                    blue -= 1;
                }
                _ => break,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut v = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut v);
        assert!(v == vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_2() {
        let mut v = vec![2, 0, 1];
        Solution::sort_colors(&mut v);
        assert!(v == vec![0, 1, 2]);
    }

    #[test]
    fn test_3() {
        let mut v = vec![0];
        Solution::sort_colors(&mut v);
        assert!(v == vec![0]);
    }

    #[test]
    fn test_4() {
        let mut v = vec![2];
        Solution::sort_colors(&mut v);
        assert!(v == vec![2]);
    }
}
