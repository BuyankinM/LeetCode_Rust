// 2643. Row With Maximum Ones
// https://leetcode.com/problems/row-with-maximum-ones/

use crate::Solution;

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![-1, -1];
        for (i, row) in mat.into_iter().enumerate() {
            let ones = row.iter().filter(|&&x| x == 1).count() as i32;
            if ones > res[1] {
                res[0] = i as i32;
                res[1] = ones;
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
            vec![0, 1],
            Solution::row_and_maximum_ones(vec![vec![0, 1], vec![1, 0]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![1, 2],
            Solution::row_and_maximum_ones(vec![vec![0, 0, 0], vec![0, 1, 1]])
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            vec![1, 2],
            Solution::row_and_maximum_ones(vec![vec![0, 0], vec![1, 1], vec![0, 0]])
        );
    }
}
