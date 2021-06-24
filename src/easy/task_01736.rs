// 1736. Latest Time by Replacing Hidden Digits
// https://leetcode.com/problems/latest-time-by-replacing-hidden-digits/

use crate::Solution;

impl Solution {
    pub fn maximum_time(time: String) -> String {
        let mut res_time = String::new();
        let time_bytes = time.as_bytes();

        for (i, &c) in time_bytes.iter().enumerate() {
            let c_res = if c == b'?' {
                if i == 0 {
                    if time_bytes[1] == b'?' || time_bytes[1] < b'4' {
                        '2'
                    } else {
                        '1'
                    }
                } else if i == 1 {
                    if time_bytes[0] == b'?' || time_bytes[0] == b'2' {
                        '3'
                    } else {
                        '9'
                    }
                } else if i == 3 {
                    '5'
                } else {
                    '9'
                }
            } else {
                c as char
            };

            res_time.push(c_res);
        }

        res_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "23:50".to_owned(),
            Solution::maximum_time("2?:?0".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "09:39".to_owned(),
            Solution::maximum_time("0?:3?".to_owned())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "19:22".to_owned(),
            Solution::maximum_time("1?:22".to_owned())
        );
    }
}
