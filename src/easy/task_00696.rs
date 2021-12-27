// 696. Count Binary Substrings
// https://leetcode.com/problems/count-binary-substrings/

use crate::Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut res = 0;
        let mut prev_b = b' ';
        let (mut prev_num, mut cur_num) = (0, 0);
        s.as_bytes().iter().for_each(|&b| {
            if prev_b != b {
                res += cur_num.min(prev_num);
                prev_num = cur_num;
                cur_num = 0;
            }
            cur_num += 1;
            prev_b = b;
        });
        res + cur_num.min(prev_num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::count_binary_substrings("00110011".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, Solution::count_binary_substrings("10101".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::count_binary_substrings("10".to_string()));
    }

    #[test]
    fn test_4() {
        assert_eq!(0, Solution::count_binary_substrings("0".to_string()));
    }
}
