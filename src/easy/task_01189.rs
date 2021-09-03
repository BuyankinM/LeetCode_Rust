// 1189. Maximum Number of Balloons
// https://leetcode.com/problems/maximum-number-of-balloons/

use crate::Solution;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        use std::collections::HashMap;

        let mut counter: HashMap<char, i32> = "balloon".chars().map(|c| (c, 0)).collect();

        text.chars().for_each(|c| {
            counter.entry(c).and_modify(|e| *e += 1);
        });

        counter
            .iter()
            .map(|(&c, &num)| num / if "lo".contains(c) { 2 } else { 1 })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::max_number_of_balloons("nlaebolko".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            2,
            Solution::max_number_of_balloons("loonbalxballpoon".to_owned())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::max_number_of_balloons("leetcode".to_owned()));
    }

    #[test]
    fn test_4() {
        assert_eq!(0, Solution::max_number_of_balloons("lloon".to_owned()));
    }
}
