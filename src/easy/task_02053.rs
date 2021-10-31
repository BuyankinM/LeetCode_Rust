// 2053. Kth Distinct String in an Array
// https://leetcode.com/problems/kth-distinct-string-in-an-array/

use crate::Solution;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut hm = std::collections::HashMap::new();
        arr.iter().enumerate().for_each(|(ind, s)| {
            let e = hm.entry(s).or_insert_with(|| vec![0, ind]);
            (*e)[0] += 1;
        });

        let mut v: Vec<(&String, Vec<usize>)> = hm.into_iter().filter(|(_, v)| v[0] == 1).collect();
        v.sort_unstable_by_key(|(_, v)| v[1]);
        v.iter()
            .map(|(s, _)| *s)
            .nth((k - 1) as usize)
            .unwrap_or(&"".to_string())
            .to_string()
    }

    pub fn kth_distinct_short(arr: Vec<String>, k: i32) -> String {
        let mut hm = std::collections::HashMap::new();
        arr.iter().for_each(|s| {
            *hm.entry(s).or_insert(0) += 1;
        });

        arr.iter()
            .filter(|&s| hm[s] == 1)
            .nth((k - 1) as usize)
            .unwrap_or(&"".to_string())
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "a".to_string(),
            Solution::kth_distinct(
                vec![
                    "d".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "a".to_string()
                ],
                2
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "aaa".to_string(),
            Solution::kth_distinct(
                vec!["aaa".to_string(), "aa".to_string(), "a".to_string()],
                1
            )
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "".to_string(),
            Solution::kth_distinct(vec!["a".to_string(), "b".to_string(), "a".to_string()], 3)
        );
    }
}
