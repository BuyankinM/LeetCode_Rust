// 1784. Check if Binary String Has at Most One Segment of Ones
// https://leetcode.com/problems/check-if-binary-string-has-at-most-one-segment-of-ones/

use crate::Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        (&s).split('0').filter(|x| !x.is_empty()).count() == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(!Solution::check_ones_segment("1001".to_owned()));
    }

    #[test]
    fn test_2() {
        assert!(Solution::check_ones_segment("110".to_owned()));
    }
}
