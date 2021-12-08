// 766. Toeplitz Matrix
// https://leetcode.com/problems/toeplitz-matrix/

use crate::Solution;

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let l = matrix.len();
        if l == 1 || matrix[0].len() == 1 {
            return true;
        }

        for (i, row) in matrix[..l - 1].iter().enumerate() {
            let next_row_it = matrix[i + 1][1..].iter();
            if row.iter().zip(next_row_it).any(|(x, y)| *x != *y) {
                return false;
            }
        }
        true
    }

    // https://leetcode.com/problems/toeplitz-matrix/discuss/922734/Rust-oneliner
    pub fn is_toeplitz_matrix_one_liner(matrix: Vec<Vec<i32>>) -> bool {
        matrix
            .windows(2)
            .all(|win| win[0][0..win[0].len() - 1] == win[1][1..win[1].len()])
    }

    // https://leetcode.com/problems/toeplitz-matrix/discuss/938520/Rust-Solution-fully-functionnal
    pub fn is_toeplitz_matrix_one_liner_zip(matrix: Vec<Vec<i32>>) -> bool {
        matrix
            .iter()
            .skip(1)
            .zip(matrix.iter())
            .all(|(current, last)| current.iter().skip(1).eq(last.iter().take(last.len() - 1)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_toeplitz_matrix(vec![
            vec![1, 2, 3, 4],
            vec![5, 1, 2, 3],
            vec![9, 5, 1, 2]
        ]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_toeplitz_matrix(vec![vec![1, 2], vec![2, 2]]));
    }

    #[test]
    fn test_3() {
        assert!(Solution::is_toeplitz_matrix(vec![vec![1, 2]]));
    }

    #[test]
    fn test_4() {
        assert!(Solution::is_toeplitz_matrix(vec![vec![1]]));
    }
}
