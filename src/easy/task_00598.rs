// 598. Range Addition II
// https://leetcode.com/problems/range-addition-ii/

use crate::Solution;

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        if ops.is_empty() {
            return m * n;
        }

        let (mut min_m, mut min_n) = (i32::MAX, i32::MAX);
        ops.iter().for_each(|op| {
            if let &[a, b] = op.as_slice() {
                min_m = min_m.min(a);
                min_n = min_n.min(b);
            }
        });
        min_m * min_n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::max_count(3, 3, vec![vec![2, 2], vec![3, 3]]));
    }

    #[test]
    fn test_2() {
        assert_eq!(9, Solution::max_count(3, 3, vec![vec![0; 0]; 0]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            4,
            Solution::max_count(
                3,
                3,
                vec![
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3],
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3],
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3]
                ]
            )
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            1,
            Solution::max_count(
                3,
                3,
                vec![
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3],
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3],
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3],
                    vec![1, 1],
                ]
            )
        );
    }
}
