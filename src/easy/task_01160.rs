// 1160. Find Words That Can Be Formed by Characters
// https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/

use crate::Solution;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let get_counter = |word: &String| -> [i32; 26] {
            let mut counter = [0; 26];
            word.as_bytes().iter().for_each(|&b| {
                counter[(b - b'a') as usize] += 1;
            });
            counter
        };

        let counter = get_counter(&chars);
        words
            .iter()
            .map(|word| {
                if get_counter(word)
                    .iter()
                    .enumerate()
                    .all(|(ind, num)| counter[ind] >= *num)
                {
                    word.len() as i32
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            6,
            Solution::count_characters(
                vec![
                    "cat".to_string(),
                    "bt".to_string(),
                    "hat".to_string(),
                    "tree".to_string()
                ],
                "atach".to_string()
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            10,
            Solution::count_characters(
                vec![
                    "hello".to_string(),
                    "world".to_string(),
                    "leetcode".to_string()
                ],
                "welldonehoneyr".to_string()
            )
        );
    }
}
