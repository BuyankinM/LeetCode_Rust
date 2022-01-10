// 2129. Capitalize the Title
// https://leetcode.com/problems/capitalize-the-title/

use crate::Solution;

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        title
            .split_whitespace()
            .map(|w| match w.len() {
                1 | 2 => w.to_ascii_lowercase(),
                _ => w[..1].to_ascii_uppercase() + &w[1..].to_ascii_lowercase(),
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "Capitalize The Title".to_string(),
            Solution::capitalize_title("capiTalIze tHe titLe".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "First Letter of Each Word".to_string(),
            Solution::capitalize_title("First leTTeR of EACH Word".to_string())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "i Love Leetcode".to_string(),
            Solution::capitalize_title("i lOve leetcode".to_string())
        );
    }
}
