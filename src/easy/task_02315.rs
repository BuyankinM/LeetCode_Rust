// 2315. Count Asterisks
// https://leetcode.com/problems/count-asterisks/

use crate::Solution;

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        s.split('|')
            .enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .flat_map(|(_, s)| s.chars())
            .filter(|&c| c == '*')
            .count() as _
    }

    pub fn count_asterisks_v2(s: String) -> i32 {
        s.split('|')
            .enumerate()
            .filter_map(|(i, s)| if i % 2 == 0 { Some(s.chars()) } else { None })
            .flatten()
            .filter(|&c| c == '*')
            .count() as _
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::count_asterisks("l|*e*et|c**o|*de|".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::count_asterisks("iamprogrammer".to_string()));
    }
    #[test]
    fn test_3() {
        assert_eq!(
            5,
            Solution::count_asterisks("yo|uar|e**|b|e***au|tifu|l".to_string())
        );
    }
}
