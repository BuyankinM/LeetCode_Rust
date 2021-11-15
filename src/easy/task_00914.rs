// 914. X of a Kind in a Deck of Cards
// https://leetcode.com/problems/x-of-a-kind-in-a-deck-of-cards/

use crate::Solution;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        fn gcd(a: i32, b: i32) -> i32 {
            match b > 0 {
                true => gcd(b, a % b),
                false => a,
            }
        }

        let mut map = std::collections::HashMap::new();
        deck.iter().for_each(|&x| *map.entry(x).or_insert(0) += 1);

        let mut prev_gcd = None;
        for &val in map.values() {
            match (val, prev_gcd) {
                (1, _) => return false,
                (_, Some(v)) => match gcd(v, val) {
                    1 => return false,
                    new_gcd => prev_gcd = Some(new_gcd),
                },
                _ => prev_gcd = Some(val),
            }
        }
        true
    }

    pub fn has_groups_size_x_fp_reduce(deck: Vec<i32>) -> bool {
        fn gcd(a: i32, b: i32) -> i32 {
            match b > 0 {
                true => gcd(b, a % b),
                false => a,
            }
        }

        let mut map = std::collections::HashMap::new();
        deck.iter().for_each(|&x| *map.entry(x).or_insert(0) += 1);
        map.values().cloned().reduce(gcd).unwrap() > 1
    }

    pub fn has_groups_size_x_fp_tryfold(deck: Vec<i32>) -> bool {
        fn gcd(a: i32, b: i32) -> i32 {
            match b > 0 {
                true => gcd(b, a % b),
                false => a,
            }
        }

        let mut map = std::collections::HashMap::new();
        deck.iter().for_each(|&x| *map.entry(x).or_insert(0) += 1);
        map.values()
            .try_fold(0, |acc, &x| match gcd(x, acc) {
                1 => None,
                val => Some(val),
            })
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::has_groups_size_x(vec![1, 2, 3, 4, 4, 3, 2, 1]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::has_groups_size_x(vec![1]));
    }

    #[test]
    fn test_4() {
        assert!(Solution::has_groups_size_x(vec![1, 1]));
    }

    #[test]
    fn test_5() {
        assert!(Solution::has_groups_size_x(vec![1, 1, 2, 2, 2, 2]));
    }

    #[test]
    fn test_6() {
        assert!(Solution::has_groups_size_x_fp_tryfold(vec![
            1, 1, 1, 1, 2, 2, 2, 2, 2, 2
        ]));
    }
}
