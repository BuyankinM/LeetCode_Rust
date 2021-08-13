// 1941. Check if All Characters Have Equal Number of Occurrences
// https://leetcode.com/problems/check-if-all-characters-have-equal-number-of-occurrences/

use crate::Solution;

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut last_equal = 0;
        let arr_count = s.as_bytes().iter().fold([0; 26], |mut acc, b| {
            let ind = (*b - b'a') as usize;
            acc[ind] += 1;
            last_equal = acc[ind];
            acc
        });

        arr_count
            .iter()
            .filter(|x| **x > 0)
            .all(|x| *x == last_equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!( Solution::are_occurrences_equal("abacbc".to_owned()));
    }

    #[test]
    fn test_2() {
        assert!(! Solution::are_occurrences_equal("aaabb".to_owned()));
    }

    #[test]
    fn test_3() {
        assert!( Solution::are_occurrences_equal("aaa".to_owned()));
    }
}
