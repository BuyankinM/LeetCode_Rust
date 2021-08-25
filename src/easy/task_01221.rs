// 1221. Split a String in Balanced Strings
// https://leetcode.com/problems/split-a-string-in-balanced-strings/

use crate::Solution;

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        s.chars()
            .fold((0, 0), |(res, mut counter), c| {
                counter += if c == 'L' { 1 } else { -1 };
                (res + (counter == 0) as i32, counter)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::balanced_string_split("RLRRLLRLRL".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::balanced_string_split("RLLLLRRRLR".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::balanced_string_split("LLLLRRRR".to_string()));
    }

    #[test]
    fn test_4() {
        assert_eq!(2, Solution::balanced_string_split("RLRRRLLRLL".to_string()));
    }
}
