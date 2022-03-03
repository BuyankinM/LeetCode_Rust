// 506. Relative Ranks
// https://leetcode.com/problems/relative-ranks/

use crate::Solution;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let l = score.len();

        let mut score_sort = score.clone();
        score_sort.sort_unstable();

        let m1 = score_sort.pop().unwrap_or(-1);
        let m2 = score_sort.pop().unwrap_or(-1);
        let m3 = score_sort.pop().unwrap_or(-1);

        score
            .iter()
            .map(|&x| match x {
                _ if x == m1 => "Gold Medal".to_string(),
                _ if x == m2 => "Silver Medal".to_string(),
                _ if x == m3 => "Bronze Medal".to_string(),
                _ => score_sort
                    .binary_search(&x)
                    .map_or(0, |i| l - i)
                    .to_string(),
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
            vec![
                "Gold Medal".to_string(),
                "Silver Medal".to_string(),
                "Bronze Medal".to_string(),
                "4".to_string(),
                "5".to_string()
            ],
            Solution::find_relative_ranks(vec![5, 4, 3, 2, 1])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![
                "Gold Medal".to_string(),
                "5".to_string(),
                "Bronze Medal".to_string(),
                "Silver Medal".to_string(),
                "4".to_string()
            ],
            Solution::find_relative_ranks(vec![10, 3, 8, 9, 4])
        );
    }
}
