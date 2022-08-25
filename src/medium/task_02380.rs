// 2380. Time Needed to Rearrange a Binary String
// https://leetcode.com/problems/time-needed-to-rearrange-a-binary-string/

use crate::Solution;

impl Solution {
    pub fn seconds_to_remove_occurrences(s: String) -> i32 {
        s.chars()
            .fold((0, 0), |(zeros, time), c| match c == '0' {
                true => (zeros + 1, time),
                false if zeros > 0 => (zeros, zeros.max(time + 1)),
                _ => (zeros, time),
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::seconds_to_remove_occurrences("0110101".to_string()),
            4
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::seconds_to_remove_occurrences("11100".to_string()),
            0
        );
    }
}
