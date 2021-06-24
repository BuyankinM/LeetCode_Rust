// 1773. Count Items Matching a Rule
// https://leetcode.com/problems/count-items-matching-a-rule/

use crate::Solution;

impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let ind: usize = match &rule_key[..] {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => 3,
        };

        items.iter().filter(|item| item[ind] == rule_value).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            1,
            Solution::count_matches(
                vec![
                    vec!["phone".to_owned(), "blue".to_owned(), "pixel".to_owned()],
                    vec![
                        "computer".to_owned(),
                        "silver".to_owned(),
                        "lenovo".to_owned()
                    ],
                    vec!["phone".to_owned(), "gold".to_owned(), "iphone".to_owned()]
                ],
                "color".to_owned(),
                "silver".to_owned()
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            2,
            Solution::count_matches(
                vec![
                    vec!["phone".to_owned(), "blue".to_owned(), "pixel".to_owned()],
                    vec![
                        "computer".to_owned(),
                        "silver".to_owned(),
                        "phone".to_owned()
                    ],
                    vec!["phone".to_owned(), "gold".to_owned(), "iphone".to_owned()]
                ],
                "type".to_owned(),
                "phone".to_owned()
            )
        );
    }
}
