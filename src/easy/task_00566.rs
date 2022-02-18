// 566. Reshape the Matrix
// https://leetcode.com/problems/reshape-the-matrix/

use crate::Solution;

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let (rows, cols) = (mat.len(), mat[0].len());
        if rows * cols != (r * c) as usize {
            return mat;
        }

        let mut it = mat.iter().flatten();
        (0..r)
            .map(|_| (0..c).map(|_| *it.next().unwrap()).collect())
            .collect()
    }

    // https://leetcode.com/problems/reshape-the-matrix/discuss/1317729/Rust-solution
    pub fn matrix_reshape_flatmap(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if mat.len() * mat[0].len() != (r * c) as usize {
            return mat;
        }
        let mut answer = vec![vec![0; c as usize]; r as usize];
        let mut values = mat.iter().flatten();
        for col in answer.iter_mut().flat_map(|row| row.iter_mut()) {
            if let Some(&val) = values.next() {
                *col = val;
            }
        }
        answer
    }

    // https://leetcode.com/problems/reshape-the-matrix/discuss/1317492/Rust%3A-4ms
    pub fn matrix_reshape_chunks(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let (r, c) = (r as usize, c as usize);
        let (m, n) = (mat.len(), mat[0].len());
        if r * c != m * n {
            mat
        } else {
            mat.iter()
                .flat_map(|r| r.iter().copied())
                .collect::<Vec<_>>()
                .chunks(c)
                .map(|r| r.to_vec())
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![vec![1, 2, 3, 4]],
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![vec![1, 2], vec![3, 4]],
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4)
        );
    }
}
