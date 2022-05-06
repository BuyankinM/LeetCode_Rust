// 1209. Remove All Adjacent Duplicates in String II
// https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string-ii/

use crate::Solution;

impl Solution {
    pub fn remove_duplicates_str_stack(s: String, k: i32) -> String {
        let mut stack = vec![];
        for c in s.chars() {
            let last = stack.last_mut();
            match last {
                Some((c_last, k_last)) if *c_last == c => {
                    *k_last += 1;
                    if *k_last == k {
                        stack.pop();
                    }
                }
                _ => stack.push((c, 1)),
            }
        }
        stack
            .into_iter()
            .flat_map(|(c, n)| [c].repeat(n as usize))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::remove_duplicates_str_stack("abcd".to_string(), 2),
            "abcd".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::remove_duplicates_str_stack("deeedbbcccbdaa".to_string(), 3),
            "aa".to_string()
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::remove_duplicates_str_stack("pbbcggttciiippooaais".to_string(), 2),
            "ps".to_string()
        );
    }
}
