// 1869. Longer Contiguous Segments of Ones than Zeros
// https://leetcode.com/problems/longer-contiguous-segments-of-ones-than-zeros/

use crate::Solution;

impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let (mut max_1, mut max_0) = (0, 0);
        let (mut cur_1, mut cur_0) = (0, 0);
        s.chars().for_each(|c| match c == '1' {
            true => {
                cur_0 = 0;
                cur_1 += 1;
                max_1 = max_1.max(cur_1)
            }
            false => {
                cur_1 = 0;
                cur_0 += 1;
                max_0 = max_0.max(cur_0)
            }
        });
        max_1 > max_0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::check_zero_ones("1101".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(false, Solution::check_zero_ones("111000".to_owned()));
    }

    #[test]
    fn test_3() {
        assert_eq!(false, Solution::check_zero_ones("110100010".to_owned()));
    }
}
