// 1582. Special Positions in a Binary Matrix
// https://leetcode.com/problems/special-positions-in-a-binary-matrix/

use crate::Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut columns_check = vec![false; mat[0].len()];

        for row in &mat {
            if columns_check.iter().all(|&x| x) {
                break;
            }

            let ones_in_row = row
                .iter()
                .enumerate()
                .filter(|&(_, x)| *x == 1)
                .collect::<Vec<(_, _)>>();

            if ones_in_row.len() != 1 {
                ones_in_row
                    .iter()
                    .for_each(|&(ind, _)| columns_check[ind] = true);
                continue;
            }

            let (ind, _) = ones_in_row[0];
            if columns_check[ind] {
                continue;
            }

            let sum_in_column = mat.iter().map(|x| x[ind]).sum::<i32>();
            if sum_in_column == 1 {
                res += 1;
            }

            columns_check[ind] = true;
        }
        res
    }

    pub fn num_special_short(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut rs = vec![0; m];
        let mut cs = vec![0; n];
        for i in 0..m {
            for (j, val) in cs.iter_mut().enumerate() {
                rs[i] += mat[i][j];
                *val += mat[i][j];
            }
        }
        let mut ans = 0;
        for i in 0..m {
            for (j, val) in cs.iter().enumerate() {
                if mat[i][j] == 1 && rs[i] <= 1 && *val <= 1 {
                    ans += 1;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            1,
            Solution::num_special(vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::num_special(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            2,
            Solution::num_special_short(vec![
                vec![0, 0, 0, 1],
                vec![1, 0, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 0]
            ])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            3,
            Solution::num_special_short(vec![
                vec![0, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 1]
            ])
        );
    }
}
