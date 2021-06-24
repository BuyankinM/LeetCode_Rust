// 7. Reverse Integer
// https://leetcode.com/problems/reverse-integer/

use crate::Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let (mut y, mut res): (i32, i32) = (x, 0);
        while y != 0 {
            let (check_over_res, over_flag_mul) = res.overflowing_mul(10);
            let (check_over_res, over_flag_add) = check_over_res.overflowing_add(y % 10);

            match over_flag_mul || over_flag_add {
                false => y /= 10,
                true => return 0,
            };

            res = check_over_res;
        }
        res
    }

    pub fn reverse_compact(x: i32) -> i32 {
        fn helper(mut n: i32) -> Option<i32> {
            let mut res = 0i32;
            while n != 0 {
                res = res.checked_mul(10)?.checked_add(n % 10)?;
                n /= 10;
            }
            Some(res)
        }
        helper(x).unwrap_or_default()
    }

    pub fn reverse_string(x: i32) -> i32 {
        x.signum()
            * x.to_string()
                .chars()
                .filter(|&c| c.is_ascii_digit())
                .rev()
                .collect::<String>()
                .parse::<i32>()
                .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(321, Solution::reverse(123));
    }

    #[test]
    fn test_2() {
        assert_eq!(-321, Solution::reverse(-123));
    }

    #[test]
    fn test_3() {
        assert_eq!(21, Solution::reverse(120));
    }

    #[test]
    fn test_4() {
        assert_eq!(0, Solution::reverse(0));
    }

    #[test]
    fn test_5() {
        assert_eq!(0, Solution::reverse(1534236469));
    }

    #[test]
    fn test_6() {
        assert_eq!(0, Solution::reverse(i32::MIN));
    }

    #[test]
    fn test_7() {
        assert_eq!(0, Solution::reverse_compact(i32::MIN));
    }

    #[test]
    fn test_8() {
        assert_eq!(0, Solution::reverse_string(i32::MIN));
    }
}
