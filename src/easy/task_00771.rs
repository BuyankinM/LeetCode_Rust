// 771. Jewels and Stones
// https://leetcode.com/problems/jewels-and-stones/

use crate::Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let jewels_set = jewels.chars().collect::<std::collections::HashSet<char>>();
        stones.chars().filter(|c| jewels_set.contains(c)).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::num_jewels_in_stones("aA".to_owned(), "aAAbbbb".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            0,
            Solution::num_jewels_in_stones("z".to_owned(), "ZZ".to_owned())
        );
    }
}
