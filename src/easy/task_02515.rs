// 2515. Shortest Distance to Target String in a Circular Array
// https://leetcode.com/problems/shortest-distance-to-target-string-in-a-circular-array/

use crate::Solution;

impl Solution {
    pub fn closet_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let mut result = -1;
        for i in words
            .iter()
            .enumerate()
            .filter_map(|(i, w)| (w == &target).then_some(i as i32))
        {
            let min_pos = i.min(start_index);
            let max_pos = i.max(start_index);
            let dist = (max_pos - min_pos).min((words.len() as i32 - max_pos) + min_pos);
            result = if result == -1 { dist } else { result.min(dist) }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            1,
            Solution::closet_target(
                vec![
                    "hello".to_string(),
                    "i".to_string(),
                    "am".to_string(),
                    "leetcode".to_string(),
                    "hello".to_string()
                ],
                "hello".to_string(),
                1
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            1,
            Solution::closet_target(
                vec!["a".to_string(), "b".to_string(), "leetcode".to_string()],
                "leetcode".to_string(),
                0
            )
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            -1,
            Solution::closet_target(
                vec!["i".to_string(), "eat".to_string(), "leetcode".to_string()],
                "ate".to_string(),
                0
            )
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            1,
            Solution::closet_target(
                vec![
                    "odjrjznxpn".to_string(),
                    "cyulttuabe".to_string(),
                    "zqxkdoeszk".to_string(),
                    "yeewpgriok".to_string(),
                    "odjrjznxpn".to_string(),
                    "btqpvxpjzv".to_string(),
                    "ukyudladhk".to_string(),
                    "ukyudladhk".to_string(),
                    "odjrjznxpn".to_string(),
                    "yeewpgriok".to_string()
                ],
                "odjrjznxpn".to_string(),
                5
            )
        );
    }
}
