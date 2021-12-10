// 748. Shortest Completing Word
// https://leetcode.com/problems/shortest-completing-word/

use crate::Solution;

impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut counter_lic = [0; 26];
        license_plate
            .chars()
            .filter_map(|c| {
                if c.is_alphabetic() {
                    Some(c.to_ascii_lowercase())
                } else {
                    None
                }
            })
            .for_each(|c| counter_lic[((c as u8) - b'a') as usize] += 1);

        let mut short_word = String::new();
        for word in words.into_iter() {
            let mut counter_word = [0; 26];
            word.as_bytes()
                .iter()
                .for_each(|&b| counter_word[(b - b'a') as usize] += 1);

            if counter_lic
                .iter()
                .zip(counter_word.iter())
                .all(|(l, w)| *w >= *l)
                && (short_word.is_empty() || short_word.len() > word.len())
            {
                short_word = word;
            }
        }
        short_word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "steps".to_string(),
            Solution::shortest_completing_word(
                "1s3 PSt".to_string(),
                vec![
                    "step".to_string(),
                    "steps".to_string(),
                    "stripe".to_string(),
                    "stepple".to_string()
                ]
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "pest".to_string(),
            Solution::shortest_completing_word(
                "1s3 456".to_string(),
                vec![
                    "looks".to_string(),
                    "pest".to_string(),
                    "stew".to_string(),
                    "show".to_string()
                ]
            )
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "husband".to_string(),
            Solution::shortest_completing_word(
                "Ah71752".to_string(),
                vec![
                    "suggest".to_string(),
                    "letter".to_string(),
                    "of".to_string(),
                    "husband".to_string(),
                    "easy".to_string(),
                    "education".to_string(),
                    "drug".to_string(),
                    "prevent".to_string(),
                    "writer".to_string(),
                    "old".to_string()
                ]
            )
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            "enough".to_string(),
            Solution::shortest_completing_word(
                "OgEu755".to_string(),
                vec![
                    "enough".to_string(),
                    "these".to_string(),
                    "play".to_string(),
                    "wide".to_string(),
                    "wonder".to_string(),
                    "box".to_string(),
                    "arrive".to_string(),
                    "money".to_string(),
                    "tax".to_string(),
                    "thus".to_string(),
                ]
            )
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            "simple".to_string(),
            Solution::shortest_completing_word(
                "iMSlpe4".to_string(),
                vec![
                    "claim".to_string(),
                    "consumer".to_string(),
                    "student".to_string(),
                    "camera".to_string(),
                    "public".to_string(),
                    "never".to_string(),
                    "wonder".to_string(),
                    "simple".to_string(),
                    "thought".to_string(),
                    "use".to_string(),
                ]
            )
        );
    }
}
