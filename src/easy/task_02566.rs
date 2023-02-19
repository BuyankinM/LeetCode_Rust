// 2566. Maximum Difference by Remapping a Digit
// https://leetcode.com/problems/maximum-difference-by-remapping-a-digit/

use crate::Solution;

impl Solution {
    pub fn min_max_difference(mut num: i32) -> i32 {
        let (mut max_val, mut min_val) = (0, 0);
        let (mut max_dig_rm, mut min_dig_rm) = (-1, -1);
        let p = (num as f64).log10() as u32;
        let mut div = 10_i32.pow(p);

        for _ in 0..=p {
            let dig = num / div;
            max_val *= 10;
            min_val *= 10;

            max_val += match (max_dig_rm, dig) {
                (-1, x) => {
                    if x != 9 {
                        max_dig_rm = x
                    }
                    9
                }
                (x, y) if x == y => 9,
                _ => dig,
            };

            min_val += match (min_dig_rm, dig) {
                (-1, x) => {
                    if x != 0 {
                        min_dig_rm = x
                    }
                    0
                }
                (x, y) if x == y => 0,
                _ => dig,
            };

            num %= div;
            div /= 10;
        }

        max_val - min_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(99009, Solution::min_max_difference(11891));
    }

    #[test]
    fn test_2() {
        assert_eq!(99, Solution::min_max_difference(90));
    }
}
