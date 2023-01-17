// 926. Flip String to Monotone Increasing
// https://leetcode.com/problems/flip-string-to-monotone-increasing/description/

use crate::Solution;

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let (mut prev_0, mut prev_1) = (0, 0);
        for b in s.bytes() {
            prev_1 = prev_0.min(prev_1) + (b != b'1') as i32;
            prev_0 += (b != b'0') as i32;
        }
        prev_0.min(prev_1)
    }

    // https://leetcode.com/problems/flip-string-to-monotone-increasing/solutions/3061376/leetcode-the-hard-way-explained-line-by-line/?orderBy=most_votes&languageTags=rust
    pub fn min_flips_mono_incr_opt(s: String) -> i32 {
        // counters to count numbers of 0s and 1s
        let mut cnt0 = 0;
        let mut cnt1 = 0;
        // for each character
        for x in s.chars() {
            // we count the number of zeros
            if x == '0' {
                cnt0 += 1;
            }
            // or the number of ones
            else {
                cnt1 += 1;
            }
            // we can either flip all ones to zeros or
            // we can just flip the current zero
            // e.g. 0010 -> 001[1]
            // e.g. 1110000 -> [000]0000
            cnt0 = cnt0.min(cnt1);
        }
        cnt0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::min_flips_mono_incr("00110".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::min_flips_mono_incr("010110".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(2, Solution::min_flips_mono_incr("00011000".to_string()));
    }

    #[test]
    fn test_4() {
        assert_eq!(
            3,
            Solution::min_flips_mono_incr("00011000111110".to_string())
        );
    }
}
