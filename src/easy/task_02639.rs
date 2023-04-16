// 2639. Find the Width of Columns of a Grid
// https://leetcode.com/problems/find-the-width-of-columns-of-a-grid/

use crate::Solution;

impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut width;
        let mut widths = vec![0; grid[0].len()];
        for row in grid {
            for (i, &x) in row.iter().enumerate() {
                width = 1 + (1.max(x.abs()) as f32).log10() as i32;
                if x < 0 {
                    width += 1;
                };

                if i == 7 {
                    println!("{width}");
                }

                if width > widths[i] {
                    widths[i] = width;
                }
            }
        }
        widths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![3],
            Solution::find_column_width(vec![vec![1], vec![22], vec![333]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![3, 1, 2],
            Solution::find_column_width(vec![vec![-15, 1, 3], vec![15, 7, 12], vec![5, 6, -2]])
        );
    }
}
