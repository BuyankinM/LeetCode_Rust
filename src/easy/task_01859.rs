// 1859. Sorting the Sentence
// https://leetcode.com/problems/sorting-the-sentence/

use crate::Solution;

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut res_vec = s
            .split_whitespace()
            .map(|x| {
                (
                    x[..x.len() - 1].to_string(),
                    x[x.len() - 1..].parse::<u32>().unwrap(),
                )
            })
            .collect::<Vec<(_, _)>>();

        res_vec.sort_unstable_by_key(|x| x.1);
        res_vec
            .iter()
            .map(|x| x.0.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "This is a sentence".to_owned(),
            Solution::sort_sentence("is2 sentence4 This1 a3".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "Me Myself and I".to_owned(),
            Solution::sort_sentence("Myself2 Me1 I4 and3".to_owned())
        );
    }
}
