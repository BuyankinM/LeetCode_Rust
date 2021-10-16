// 1128. Number of Equivalent Domino Pairs
// https://leetcode.com/problems/number-of-equivalent-domino-pairs/

use crate::Solution;

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut table = [[0; 9]; 9];
        let combinations = |n: i32| -> i32 { (n - 1) * n / 2 };

        // construct an upper triangular matrix
        dominoes.iter().for_each(|d| {
            let (row, col) = match (d[0] - 1, d[1] - 1) {
                (a, b) if a < b => (a, b),
                (a, b) => (b, a),
            };
            table[row as usize][col as usize] += 1;
        });

        // check the elements above the main diagonal of the matrix
        for (start_col, row) in table.iter().enumerate() {
            for &num in row[start_col..].iter().filter(|&&num| num > 1) {
                res += combinations(num);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            1,
            Solution::num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::num_equiv_domino_pairs(vec![
                vec![1, 2],
                vec![1, 2],
                vec![1, 1],
                vec![1, 2],
                vec![2, 2]
            ])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            11,
            Solution::num_equiv_domino_pairs(vec![
                vec![2, 1],
                vec![5, 4],
                vec![3, 7],
                vec![6, 2],
                vec![4, 4],
                vec![1, 8],
                vec![9, 6],
                vec![5, 3],
                vec![7, 4],
                vec![1, 9],
                vec![1, 1],
                vec![6, 6],
                vec![9, 6],
                vec![1, 3],
                vec![9, 7],
                vec![4, 7],
                vec![5, 1],
                vec![6, 5],
                vec![1, 6],
                vec![6, 1],
                vec![1, 8],
                vec![7, 2],
                vec![2, 4],
                vec![1, 6],
                vec![3, 1],
                vec![3, 9],
                vec![3, 7],
                vec![9, 1],
                vec![1, 9],
                vec![8, 9]
            ])
        );
    }
}
