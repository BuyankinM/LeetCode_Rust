// 59. Spiral Matrix II
// https://leetcode.com/problems/spiral-matrix-ii/

use crate::Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let nu = n as usize;
        let mut matrix = vec![vec![0; nu]; nu];
        let (mut left, mut right, mut top, mut bottom) = (0, nu, 0, nu);
        let mut num = 1;

        while num <= n * n {
            (left..right).for_each(|i| {
                matrix[top][i] = num;
                num += 1;
            });
            top += 1;

            (top..bottom).for_each(|i| {
                matrix[i][right - 1] = num;
                num += 1;
            });
            right -= 1;

            (left..right).rev().for_each(|i| {
                matrix[bottom - 1][i] = num;
                num += 1;
            });
            bottom -= 1;

            (top..bottom).rev().for_each(|i| {
                matrix[i][left] = num;
                num += 1;
            });
            left += 1;
        }
        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![vec![1]], Solution::generate_matrix(1));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]],
            Solution::generate_matrix(3)
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![
                vec![1, 2, 3, 4, 5],
                vec![16, 17, 18, 19, 6],
                vec![15, 24, 25, 20, 7],
                vec![14, 23, 22, 21, 8],
                vec![13, 12, 11, 10, 9]
            ],
            Solution::generate_matrix(5)
        );
    }
}
