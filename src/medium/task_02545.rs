// 2545. Sort the Students by Their Kth Score
// https://leetcode.com/problems/sort-the-students-by-their-kth-score/

use crate::Solution;

impl Solution {
    pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        score.sort_unstable_by_key(|v| -v[k as usize]);
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![vec![7, 5, 11, 2], vec![10, 6, 9, 1], vec![4, 8, 3, 15]],
            Solution::sort_the_students(
                vec![vec![10, 6, 9, 1], vec![7, 5, 11, 2], vec![4, 8, 3, 15]],
                2
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![vec![5, 6], vec![3, 4]],
            Solution::sort_the_students(vec![vec![3, 4], vec![5, 6]], 0)
        );
    }
}
