// 118. Pascal's Triangle
// https://leetcode.com/problems/pascals-triangle/

use crate::Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        use std::iter::once;
        match num_rows {
            0 => vec![],
            _ => {
                let mut res = vec![vec![1]];
                for i in 1..num_rows as usize {
                    let prev_row_it = res[i - 1].iter();
                    let row = once(1)
                        .chain(
                            prev_row_it
                                .clone()
                                .zip(prev_row_it.skip(1))
                                .map(|(a, b)| a + b),
                        )
                        .chain(once(1))
                        .collect::<Vec<_>>();
                    res.push(row);
                }
                res
            }
        }
    }

    // https://leetcode.com/problems/pascals-triangle/discuss/1287184/Rust-simple-solution
    pub fn generate_short(num_rows: i32) -> Vec<Vec<i32>> {
        (0..num_rows)
            .scan(Vec::with_capacity(num_rows as usize), |state, _| {
                for i in (1..state.len()).rev() {
                    state[i] += state[i - 1];
                }
                state.push(1);
                Some(state.clone())
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::generate(0), Vec::<Vec<i32>>::new());
    }
}
