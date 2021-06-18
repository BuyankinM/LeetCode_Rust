// 1572. Matrix Diagonal Sum
// https://leetcode.com/problems/matrix-diagonal-sum/

use crate::Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let l = mat.len();
        (0..l).fold(0, |res, i| {
            res + mat[i][i]
                + if i != (l - i - 1) {
                    mat[i][l - i - 1]
                } else {
                    0
                }
        })
    }

    pub fn diagonal_sum_optimal(mat: Vec<Vec<i32>>) -> i32 {
        let l = mat.len();
        (0..l).fold(0, |res, i| res + mat[i][i] + mat[i][l - i - 1])
            - if l % 2 == 1 { mat[l / 2][l / 2] } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            25,
            Solution::diagonal_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            8,
            Solution::diagonal_sum(vec![
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1]
            ])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(5, Solution::diagonal_sum_optimal(vec![vec![5]]));
    }
}
