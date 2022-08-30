// 48. Rotate Image
// https://leetcode.com/problems/rotate-image/

use crate::Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        // transpose matrix
        for i in 0..n - 1 {
            for j in i + 1..n {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }

        // reverse
        matrix.iter_mut().for_each(|row| row.reverse());
    }

    // https://leetcode.com/problems/rotate-image/discuss/2504047/Rust-or-Three-Solutions-or-With-Comments
    pub fn rotate_opt(matrix: &mut Vec<Vec<i32>>) {
        matrix.reverse();
        for i in 1..matrix.len() {
            for j in 0..i {
                let t = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = t;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut v);
        assert_eq!(v, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    #[test]
    fn test_2() {
        let mut v = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut v);
        assert_eq!(
            v,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }
}
