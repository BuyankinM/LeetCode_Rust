// 744. Find Smallest Letter Greater Than Target
// https://leetcode.com/problems/find-smallest-letter-greater-than-target/

use crate::Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        letters[match letters.binary_search(&target) {
            Ok(i) => i + 1,
            Err(i) => i,
        } % letters.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            'c',
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'a')
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            'f',
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'c')
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            'f',
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'd')
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            'j',
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'g')
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            'c',
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'j')
        );
    }
}
