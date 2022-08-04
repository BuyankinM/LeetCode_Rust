// 2347. Best Poker Hand
// https://leetcode.com/problems/best-poker-hand/

use crate::Solution;

impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        match suits.iter().collect::<std::collections::HashSet<_>>().len() == 1 {
            true => "Flush".to_string(),
            false => {
                let mut counter = [0; 14];
                ranks.iter().for_each(|&r| counter[r as usize] += 1);
                match counter.into_iter().max().unwrap() {
                    1 => "High Card".to_string(),
                    2 => "Pair".to_string(),
                    _ => "Three of a Kind".to_string(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::best_hand(vec![13, 2, 3, 1, 9], vec!['a', 'a', 'a', 'a', 'a']),
            "Flush".to_string(),
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::best_hand(vec![4, 4, 2, 4, 4], vec!['b', 'a', 'a', 'a', 'a']),
            "Three of a Kind".to_string(),
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::best_hand(vec![10, 10, 2, 13, 13], vec!['b', 'a', 'a', 'a', 'a']),
            "Pair".to_string(),
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::best_hand(vec![10, 11, 12, 13, 9], vec!['b', 'a', 'a', 'a', 'a']),
            "High Card".to_string(),
        );
    }
}
