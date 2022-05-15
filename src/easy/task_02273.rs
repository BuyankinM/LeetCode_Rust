// 2273. Find Resultant Array After Removing Anagrams
// https://leetcode.com/problems/find-resultant-array-after-removing-anagrams/

use crate::Solution;

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut res = vec![];
        let mut prev_count = [0; 26];
        for word in words {
            let mut count = [0; 26];
            word.bytes().for_each(|b| count[(b - b'a') as usize] += 1);

            if prev_count != count {
                res.push(word);
            }
            prev_count = count;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::remove_anagrams(vec![
                "abba".to_string(),
                "baba".to_string(),
                "bbaa".to_string(),
                "cd".to_string(),
                "cd".to_string()
            ]),
            vec!["abba".to_string(), "cd".to_string()]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::remove_anagrams(vec![
                "a".to_string(),
                "a".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
                "a".to_string()
            ]),
            vec![
                "a".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
                "a".to_string()
            ]
        );
    }
}
