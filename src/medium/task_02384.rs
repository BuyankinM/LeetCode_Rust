// 2384. Largest Palindromic Number
// https://leetcode.com/problems/largest-palindromic-number/

use crate::Solution;

impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let mut res = Vec::with_capacity(num.len());
        let mut mid = None;

        let mut counter = [0; 10];
        num.bytes().for_each(|b| counter[(b - b'0') as usize] += 1);

        // first half
        for (i, n) in counter.iter().enumerate().filter(|(_, &n)| n > 0).rev() {
            let b = i as u8 + b'0';

            if mid.is_none() && n % 2 == 1 {
                mid = Some(b);
            }

            if b == b'0' {
                match (res.is_empty(), mid) {
                    (true, Some(b)) => return String::from_utf8(vec![b]).unwrap(),
                    (true, None) => return "0".to_string(),
                    _ => (),
                }
            }

            (0..n / 2).for_each(|_| res.push(b));
        }

        let l = res.len();

        // middle
        if let Some(mid) = mid {
            res.push(mid);
        }

        // mirror half
        (0..l).rev().for_each(|i| res.push(res[i]));

        String::from_utf8(res).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::largest_palindromic("444947137".to_string()),
            "7449447".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::largest_palindromic("00009".to_string()),
            "9".to_string()
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::largest_palindromic("000099".to_string()),
            "900009".to_string()
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::largest_palindromic("0000".to_string()),
            "0".to_string()
        );
    }
}
