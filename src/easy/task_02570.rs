// 2570. Merge Two 2D Arrays by Summing Values
// https://leetcode.com/problems/merge-two-2d-arrays-by-summing-values/

use crate::Solution;

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut bt = std::collections::BTreeMap::new();
        bt.extend(nums1.iter().map(|x| (x[0], x[1])));
        nums2
            .iter()
            .for_each(|x| *bt.entry(x[0]).or_insert(0) += x[1]);

        bt.into_iter().map(|(id, val)| vec![id, val]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]],
            Solution::merge_arrays(
                vec![vec![1, 2], vec![2, 3], vec![4, 5]],
                vec![vec![1, 4], vec![3, 2], vec![4, 1]]
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![vec![1, 3], vec![2, 4], vec![3, 6], vec![4, 3], vec![5, 5]],
            Solution::merge_arrays(
                vec![vec![2, 4], vec![3, 6], vec![5, 5]],
                vec![vec![1, 3], vec![4, 3]]
            )
        );
    }
}
