// 1945. Sum of Digits of String After Convert
// https://leetcode.com/problems/sum-of-digits-of-string-after-convert/

use crate::Solution;

impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut res = s.as_bytes().iter().fold(0, |acc, &b| {
            let x = (b - b'a' + 1) as i32;
            acc + (x / 10 + x % 10)
        });

        (1..k).for_each(|_| {
            let mut sum_dig = 0;
            while res > 0 {
                sum_dig += res % 10;
                res /= 10;
            }
            res = sum_dig;
        });

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(36, Solution::get_lucky("iiii".to_owned(), 1));
    }

    #[test]
    fn test_2() {
        assert_eq!(6, Solution::get_lucky("leetcode".to_owned(), 2));
    }

    #[test]
    fn test_3() {
        assert_eq!(8, Solution::get_lucky("zbax".to_owned(), 2));
    }

    #[test]
    fn test_4() {
        assert_eq!(9, Solution::get_lucky("abcdefgh".to_owned(), 5));
    }
}
