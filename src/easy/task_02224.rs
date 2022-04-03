// 2224. Minimum Number of Operations to Convert Time
// https://leetcode.com/problems/minimum-number-of-operations-to-convert-time/

use crate::Solution;

impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        if current == correct {
            return 0;
        }

        let to_num = |s: &str| -> i32 { s.parse().unwrap() };
        let (h2, m2) = (to_num(&correct[..2]), to_num(&correct[3..]));
        let (h1, m1) = (to_num(&current[..2]), to_num(&current[3..]));

        let mut delta = (m2 - m1) + 60 * if h2 >= h1 { h2 - h1 } else { h2 - h1 + 24 };
        let mut res = 0;

        for op in [60, 15, 5] {
            res += delta / op;
            delta %= op;
        }

        res + delta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::convert_time("02:30".to_string(), "04:35".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            1,
            Solution::convert_time("11:00".to_string(), "11:01".to_string())
        );
    }
}
