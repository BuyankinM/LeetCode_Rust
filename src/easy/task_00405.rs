// 405. Convert a Number to Hexadecimal
// https://leetcode.com/problems/convert-a-number-to-hexadecimal/

use crate::Solution;

impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }

        let mut n = num as u32;
        let mut res = Vec::new();
        let map = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];

        while n != 0 {
            res.push(map[(n % 16) as usize]);
            n /= 16;
        }
        res.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::to_hex(0), "0".to_string());
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::to_hex(120), "78".to_string());
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::to_hex(-120), "ffffff88".to_string());
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::to_hex(16), "10".to_string());
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::to_hex(1), "1".to_string());
    }
}
