// 2363. Merge Similar Items
// https://leetcode.com/problems/merge-similar-items/

use crate::Solution;

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut btm = std::collections::BTreeMap::new();
        items1
            .iter()
            .chain(items2.iter())
            .for_each(|pair| *btm.entry(pair[0]).or_insert(0) += pair[1]);
        btm.into_iter()
            .map(|(key, value)| vec![key, value])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::merge_similar_items(
                vec![vec![1, 1], vec![4, 5], vec![3, 8]],
                vec![vec![3, 1], vec![1, 5]]
            ),
            vec![vec![1, 6], vec![3, 9], vec![4, 5]]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::merge_similar_items(
                vec![vec![1, 1], vec![3, 2], vec![2, 3]],
                vec![vec![2, 1], vec![3, 2], vec![1, 3]]
            ),
            vec![vec![1, 4], vec![2, 4], vec![3, 4]]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::merge_similar_items(
                vec![vec![1, 3], vec![2, 2]],
                vec![vec![7, 1], vec![2, 2], vec![1, 4]]
            ),
            vec![vec![1, 7], vec![2, 4], vec![7, 1]]
        );
    }
}
