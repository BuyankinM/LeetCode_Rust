// 504. Base 7
// https://leetcode.com/problems/base-7/

use crate::Solution;

impl Solution {
    pub fn convert_to_base7(mut num: i32) -> String {
        let (mut res, mut base) = (0, 1);
        while num != 0 {
            res += base * (num % 7);
            base *= 10;
            num /= 7;
        }
        res.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("202".to_string(), Solution::convert_to_base7(100));
    }

    #[test]
    fn test_2() {
        assert_eq!("-10".to_string(), Solution::convert_to_base7(-7));
    }

    #[test]
    fn test_3() {
        assert_eq!("0".to_string(), Solution::convert_to_base7(0));
    }
}
