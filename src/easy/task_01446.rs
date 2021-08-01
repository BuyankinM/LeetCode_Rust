// 1446. Consecutive Characters
// https://leetcode.com/problems/consecutive-characters/

use crate::Solution;

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let (mut max_res, mut res) = (1, 0);
        let mut prev_char = ' ';

        for cur_char in s.chars() {
            match cur_char == prev_char {
                true => {
                    res += 1;
                    max_res = max_res.max(res);
                }
                false => {
                    res = 1;
                    prev_char = cur_char;
                }
            };
        }
        max_res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::max_power("leetcode".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(5, Solution::max_power("abbcccddddeeeeedcba".to_owned()));
    }

    #[test]
    fn test_3() {
        assert_eq!(5, Solution::max_power("triplepillooooow".to_owned()));
    }

    #[test]
    fn test_4() {
        assert_eq!(11, Solution::max_power("hooraaaaaaaaaaay".to_owned()));
    }
}
