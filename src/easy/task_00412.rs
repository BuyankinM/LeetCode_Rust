// 412. Fizz Buzz
// https://leetcode.com/problems/fizz-buzz/

use crate::Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .into_iter()
            .map(|i| match (i % 3 == 0, i % 5 == 0) {
                (true, true) => "FizzBuzz".to_string(),
                (true, false) => "Fizz".to_string(),
                (false, true) => "Buzz".to_string(),
                _ => i.to_string(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::fizz_buzz(3),
            vec!["1".to_string(), "2".to_string(), "Fizz".to_string()]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::fizz_buzz(5),
            vec![
                "1".to_string(),
                "2".to_string(),
                "Fizz".to_string(),
                "4".to_string(),
                "Buzz".to_string()
            ]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::fizz_buzz(27),
            vec![
                "1".to_string(),
                "2".to_string(),
                "Fizz".to_string(),
                "4".to_string(),
                "Buzz".to_string(),
                "Fizz".to_string(),
                "7".to_string(),
                "8".to_string(),
                "Fizz".to_string(),
                "Buzz".to_string(),
                "11".to_string(),
                "Fizz".to_string(),
                "13".to_string(),
                "14".to_string(),
                "FizzBuzz".to_string(),
                "16".to_string(),
                "17".to_string(),
                "Fizz".to_string(),
                "19".to_string(),
                "Buzz".to_string(),
                "Fizz".to_string(),
                "22".to_string(),
                "23".to_string(),
                "Fizz".to_string(),
                "Buzz".to_string(),
                "26".to_string(),
                "Fizz".to_string(),
            ]
        );
    }
}
