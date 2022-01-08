// 682. Baseball Game
// https://leetcode.com/problems/baseball-game/

use crate::Solution;

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut res = Vec::new();
        for s in ops.iter() {
            match s.as_str() {
                "C" => res.truncate(res.len() - 1),
                "D" => res.push(*res.last().unwrap() * 2),
                "+" => res.push(res.iter().rev().take(2).sum()),
                d => res.push(d.parse::<i32>().unwrap()),
            }
        }
        res.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            30,
            Solution::cal_points(vec![
                "5".to_string(),
                "2".to_string(),
                "C".to_string(),
                "D".to_string(),
                "+".to_string()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            27,
            Solution::cal_points(vec![
                "5".to_string(),
                "-2".to_string(),
                "4".to_string(),
                "C".to_string(),
                "D".to_string(),
                "9".to_string(),
                "+".to_string(),
                "+".to_string()
            ])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::cal_points(vec!["1".to_string()]));
    }
}
