// 2506. Count Pairs Of Similar Strings
// https://leetcode.com/problems/count-pairs-of-similar-strings/

use crate::Solution;

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut counter = std::collections::HashMap::new();
        for word in &words {
            // translate each word into bit-set of letters: "aabc" => 0000000000000000000000111
            let n = word.bytes().fold(0, |mask, b| mask | (1 << (b - b'a')));

            // count similar sets
            *counter.entry(n).or_insert(0) += 1;
        }

        // calc combinations with formula: C(kn) = n! / (n - k)! * k!
        // with k = 2: C(kn) = n * (n-1) / 2
        counter.values().fold(0, |acc, &x| {
            acc + match x >= 2 {
                true => x * (x - 1) / 2,
                false => 0,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::similar_pairs(vec![
                "aba".to_string(),
                "aabb".to_string(),
                "abcd".to_string(),
                "bac".to_string(),
                "aabc".to_string()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::similar_pairs(vec!["aabb".to_string(), "ab".to_string(), "ba".to_string()])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            0,
            Solution::similar_pairs(vec![
                "nba".to_string(),
                "cba".to_string(),
                "dba".to_string()
            ])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            120,
            Solution::similar_pairs(vec![
                "baacdbeadecebdeceadc".to_string(),
                "bdddebcdaeeecadddbda".to_string(),
                "bebcabbeecbeeaaaceac".to_string(),
                "bbcdebbeaccdaccbeded".to_string(),
                "daceacadabbcabdaccbe".to_string(),
                "abcbbcacbdabadebcebb".to_string(),
                "deacbcaaacabeabeacee".to_string(),
                "bbeecddcccdbeddbcadb".to_string(),
                "eeaedddebdddcdddbeab".to_string(),
                "debbeeedabeebeddeaae".to_string(),
                "cebaacdbbeccaccddbed".to_string(),
                "aaeaeccaceebdeeadbdc".to_string(),
                "ccbeeceeecdbeeaccbbe".to_string(),
                "aeeddeccacaacacbbeeb".to_string(),
                "cbabbdeabbacdaccddba".to_string(),
                "cccebdaaddccbbbbadda".to_string(),
                "dccbceaeaaabbdcbcbee".to_string(),
                "cccddebcdebebbdeacec".to_string()
            ])
        );
    }
}
