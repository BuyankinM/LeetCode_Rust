// 451. Sort Characters By Frequency
// https://leetcode.com/problems/sort-characters-by-frequency/

use crate::Solution;

impl Solution {
    pub fn frequency_sort_str(s: String) -> String {
        let mut map = std::collections::HashMap::with_capacity(26 * 2);
        s.chars().for_each(|b| *map.entry(b).or_insert(0) += 1);

        let mut v = map.into_iter().map(|(c, n)| (c, n)).collect::<Vec<_>>();
        v.sort_unstable_by_key(|&(c, num)| (-num, c));
        v.into_iter()
            .flat_map(|(c, n)| vec![c; n as usize])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "eert".to_string(),
            Solution::frequency_sort_str("tree".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "aaaccc".to_string(),
            Solution::frequency_sort_str("cccaaa".to_string())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "bbAa".to_string(),
            Solution::frequency_sort_str("Aabb".to_string())
        );
    }
}
