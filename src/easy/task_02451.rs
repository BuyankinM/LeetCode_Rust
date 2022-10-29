// 2451. Odd String Difference
// https://leetcode.com/problems/odd-string-difference/

use crate::Solution;

impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let get_diff_vec = |s: &str| -> Vec<i32> {
            s[1..]
                .bytes()
                .zip(s.bytes())
                .map(|(b, a)| b as i32 - a as i32)
                .collect()
        };

        let mut map = std::collections::HashMap::with_capacity(2);
        for word in words.into_iter() {
            let key = get_diff_vec(word.as_str());
            let (count, _) = map.entry(key).or_insert((0, word));

            *count += 1;
            if *count > 1 && map.len() > 1 {
                break;
            }
        }

        map.into_values()
            .find_map(|(count, word)| (count == 1).then_some(word))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "abc".to_string(),
            Solution::odd_string(vec![
                "adc".to_string(),
                "wzy".to_string(),
                "abc".to_string()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "bob".to_string(),
            Solution::odd_string(vec![
                "aaa".to_string(),
                "bob".to_string(),
                "ccc".to_string(),
                "ddd".to_string()
            ])
        );
    }
}
