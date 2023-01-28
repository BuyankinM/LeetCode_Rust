// 1422. Maximum Score After Splitting a String
// https://leetcode.com/problems/maximum-score-after-splitting-a-string/

use crate::Solution;

impl Solution {
    pub fn max_score_1422(s: String) -> i32 {
        let l = s.len();
        let mut sum_ones = s.chars().filter(|&c| c == '1').count();

        if sum_ones == l || sum_ones == 0 {
            return l as i32 - 1;
        }

        s[..l - 1]
            .chars()
            .fold((0, 0), |(mut sum_zeros, max_sum), c| {
                match c == '1' {
                    true => sum_ones -= 1,
                    false => sum_zeros += 1,
                };
                (sum_zeros, max_sum.max(sum_ones + sum_zeros))
            })
            .1 as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::max_score_1422("011101".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(5, Solution::max_score_1422("00111".to_owned()));
    }

    #[test]
    fn test_3() {
        assert_eq!(3, Solution::max_score_1422("1111".to_owned()));
    }

    #[test]
    fn test_4() {
        assert_eq!(2, Solution::max_score_1422("01".to_owned()));
    }

    #[test]
    fn test_5() {
        assert_eq!(1, Solution::max_score_1422("00".to_owned()));
    }
}
