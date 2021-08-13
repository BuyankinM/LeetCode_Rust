// 1323. Maximum 69 Number
// https://leetcode.com/problems/maximum-69-number/

use crate::Solution;

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let (mut max_pos_6, mut n) = (-1, 0);
        let mut number = num;

        while number > 0 {
            if number % 10 == 6 {
                max_pos_6 = n;
            }
            n += 1;
            number /= 10;
        }

        num + match max_pos_6 > -1 {
            true => 3 * 10_i32.pow(max_pos_6 as u32),
            false => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(9969, Solution::maximum69_number(9669));
    }

    #[test]
    fn test_2() {
        assert_eq!(9999, Solution::maximum69_number(9999));
    }

    #[test]
    fn test_3() {
        assert_eq!(96, Solution::maximum69_number(66));
    }
}
