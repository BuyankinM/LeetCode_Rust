// 2278. Percentage of Letter in String
// https://leetcode.com/problems/percentage-of-letter-in-string/

use crate::Solution;

impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        (s.chars().filter(|&c| c == letter).count() * 100 / s.len()) as i32
    }
}
