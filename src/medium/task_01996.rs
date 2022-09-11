// 1996. The Number of Weak Characters in the Game
// https://leetcode.com/problems/the-number-of-weak-characters-in-the-game/

use crate::Solution;

impl Solution {
    pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        properties.sort_unstable_by_key(|pair| (-pair[0], pair[1]));
        properties
            .iter()
            .fold((0, i32::MIN), |(res, max_def), pair| {
                (res + (pair[1] < max_def) as i32, (max_def.max(pair[1])))
            })
            .0
    }

    pub fn number_of_weak_characters_greedy(properties: Vec<Vec<i32>>) -> i32 {
        let mut max_defs = [0; 100_002];

        properties.iter().for_each(|pair| {
            if let &[attack, deffense] = pair.as_slice() {
                let max_def_per_attack = &mut max_defs[attack as usize];
                *max_def_per_attack = deffense.max(*max_def_per_attack);
            }
        });

        let mut prev_max_def = i32::MIN;
        max_defs.iter_mut().rev().for_each(|def| {
            *def = prev_max_def.max(*def);
            prev_max_def = *def;
        });

        properties
            .iter()
            .filter(|pair| pair[1] < max_defs[pair[0] as usize + 1])
            .count() as _
    }

    // https://leetcode.com/problems/the-number-of-weak-characters-in-the-game/discuss/2552715/Rust-solutions/1594842
    pub fn number_of_weak_characters_optimal(properties: Vec<Vec<i32>>) -> i32 {
        let mut max_defs = [0; 100_002];

        let mut it = properties.iter();
        while let Some(&[attack, defense]) = it.next().map(|pair| pair.as_slice()) {
            max_defs[attack as usize] = max_defs[attack as usize].max(defense);
        }

        max_defs.iter_mut().rev().fold(0, |max, def| {
            *def = max.max(*def);
            *def
        });

        properties
            .iter()
            .filter(|pair| pair[1] < max_defs[pair[0] as usize + 1])
            .count() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::number_of_weak_characters(vec![vec![5, 5], vec![6, 3], vec![3, 6]]),
            0
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::number_of_weak_characters(vec![vec![2, 2], vec![3, 3]]),
            1
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::number_of_weak_characters(vec![vec![1, 5], vec![10, 4], vec![4, 3]]),
            1
        );
    }
}
