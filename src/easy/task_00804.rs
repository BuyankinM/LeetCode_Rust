// 804. Unique Morse Code Words
// https://leetcode.com/problems/unique-morse-code-words/

use crate::Solution;

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let codes = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];
        let mut set = std::collections::HashSet::new();
        for word in &words {
            let mut trans = String::new();
            for s in word.as_bytes().iter().map(|&b| codes[(b - b'a') as usize]) {
                trans.push_str(s);
            }
            set.insert(trans);
        }
        set.len() as _
    }

    // https://leetcode.com/problems/unique-morse-code-words/discuss/461805/Rust-0ms-Functional-Solution
    pub fn unique_morse_representations_func(words: Vec<String>) -> i32 {
        const TABLE: [&str; 26] = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];
        words
            .iter()
            .map(|w| {
                w.chars()
                    .map(|c| TABLE[c as usize - 'a' as usize])
                    .collect::<String>()
            })
            .collect::<std::collections::HashSet<_>>()
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::unique_morse_representations(vec![
                "gin".to_string(),
                "zen".to_string(),
                "gig".to_string(),
                "msg".to_string()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            1,
            Solution::unique_morse_representations(vec!["a".to_string()])
        );
    }
}
