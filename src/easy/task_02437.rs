// 2437. Number of Valid Clock Times
// https://leetcode.com/problems/number-of-valid-clock-times/

use crate::Solution;

impl Solution {
    pub fn count_time(time: String) -> i32 {
        let mut res = 1;
        if let &[h1, h2, _, m1, m2] = time.as_bytes() {
            res *= match (h1, h2) {
                (b'?', b'?') => 24,
                (b'?', b'0'..=b'3') => 3,
                (b'?', _) => 2,
                (b'2', b'?') => 4,
                (_, b'?') => 10,
                _ => 1,
            };
            res *= if m1 == b'?' { 6 } else { 1 };
            res *= if m2 == b'?' { 10 } else { 1 };
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_time("?5:00".to_string()), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::count_time("0?:0?".to_string()), 100);
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::count_time("??:??".to_string()), 1440);
    }
}
