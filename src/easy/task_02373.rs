// 2373. Largest Local Values in a Matrix
// https://leetcode.com/problems/largest-local-values-in-a-matrix/

use crate::Solution;

const MAX_VAL: i32 = 100;

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut res = vec![vec![0; n - 2]; n - 2];
        'outer: for (i, r) in res.iter_mut().enumerate() {
            for (j, x) in r.iter_mut().enumerate() {
                for row in grid.iter().take(i + 3).skip(i) {
                    for &elem in row.iter().take(j + 3).skip(j) {
                        if elem > *x {
                            *x = elem;
                            if elem == MAX_VAL {
                                continue 'outer;
                            }
                        }
                    }
                }
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
            Solution::largest_local(vec![
                vec![9, 9, 8, 1],
                vec![5, 6, 2, 6],
                vec![8, 2, 6, 4],
                vec![6, 2, 2, 2]
            ]),
            vec![vec![9, 9], vec![8, 6]]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::largest_local(vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 2, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1]
            ]),
            vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]]
        );
    }
}
