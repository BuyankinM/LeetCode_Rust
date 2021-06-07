// 966. Vowel Spellchecker
// https://leetcode.com/problems/vowel-spellchecker/
use crate::Solution;
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        fn make_string_wo_vowels(s: &String) -> String {
            s.chars()
                .map(|x| match x.to_ascii_lowercase() {
                    'a' | 'e' | 'i' | 'o' | 'u' => '_',
                    c => c,
                })
                .collect()
        }

        let mut result = Vec::with_capacity(wordlist.len());
        let mut full_match = HashSet::new();
        let mut case_insensitive_match = HashMap::new();
        let mut vowel_errors_match = HashMap::new();

        for s in &wordlist {
            full_match.insert(s);

            case_insensitive_match.entry(s.to_lowercase()).or_insert(s);

            vowel_errors_match
                .entry(make_string_wo_vowels(s))
                .or_insert(s);
        }

        for q in &queries {
            if full_match.contains(q) {
                result.push(q.to_owned())
            } else if let Some(&s) = case_insensitive_match.get(&q.to_lowercase()) {
                result.push(s.to_owned())
            } else if let Some(&s) = vowel_errors_match.get(&make_string_wo_vowels(&q)) {
                result.push(s.to_owned())
            } else {
                result.push("".to_owned())
            }
        }

        result
    }
}
