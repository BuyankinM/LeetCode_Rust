// 2225. Find Players With Zero or One Losses
// https://leetcode.com/problems/find-players-with-zero-or-one-losses/

use crate::Solution;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut map = std::collections::BTreeMap::new();
        for pair in &matches {
            map.entry(pair[0]).or_insert(1);
            map.entry(pair[1])
                .and_modify(|l| *l = (-1).min(*l - 1))
                .or_insert(-1);
        }

        let (mut wins, mut losses) = (Vec::new(), Vec::new());
        for (key, val) in map.into_iter().filter(|&(_, val)| val >= -1) {
            match val > 0 {
                true => wins.push(key),
                false => losses.push(key),
            }
        }
        vec![wins, losses]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_winners(vec![
                vec![1, 3],
                vec![2, 3],
                vec![3, 6],
                vec![5, 6],
                vec![5, 7],
                vec![4, 5],
                vec![4, 8],
                vec![4, 9],
                vec![10, 4],
                vec![10, 9]
            ]),
            vec![vec![1, 2, 10], vec![4, 5, 7, 8]]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::find_winners(vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]]),
            vec![vec![1, 2, 5, 6], vec![]]
        );
    }
}
