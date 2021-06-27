// 1876. Substrings of Size Three with Distinct Characters
// https://leetcode.com/problems/substrings-of-size-three-with-distinct-characters/

use crate::Solution;

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let sb = s.as_bytes();
        sb.iter().enumerate().skip(2).fold(0, |res, (ind, b)| {
            res + (*b != sb[ind - 1] && *b != sb[ind - 2] && sb[ind - 1] != sb[ind - 2]) as i32
        })
    }

    pub fn count_good_substrings_wind(s: String) -> i32 {
        s.as_bytes()
            .windows(3)
            .filter(|w| w[0] != w[1] && w[0] != w[2] && w[1] != w[2])
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::count_good_substrings("xyzzaz".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            4,
            Solution::count_good_substrings_wind("aababcabc".to_owned())
        );
    }
}
