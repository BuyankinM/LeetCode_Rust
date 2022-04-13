// 434. Number of Segments in a String
// https://leetcode.com/problems/number-of-segments-in-a-string/

use crate::Solution;

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        s.split_ascii_whitespace().count() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            5,
            Solution::count_segments("Hello, my name is John".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::count_segments("Hello".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            5,
            Solution::count_segments("Hello,    my  name    is  John".to_string())
        );
    }
}
