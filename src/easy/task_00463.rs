// 463. Island Perimeter
// https://leetcode.com/problems/island-perimeter/

use crate::Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut perimeter = 0;
        let mut prev_row = &vec![];

        for (ind_row, row) in grid.iter().enumerate() {
            let mut prev_ind_col = -2;

            for (ind_col, _) in row.iter().enumerate().filter(|(_, &val)| val == 1) {
                perimeter += 4;

                if ind_row > 0 && prev_row[ind_col] == 1 {
                    perimeter -= 2;
                }

                if (ind_col as i32 - prev_ind_col) == 1 {
                    perimeter -= 2;
                }

                prev_ind_col = ind_col as i32;
            }

            prev_row = row;
        }

        perimeter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            16,
            Solution::island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0]
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(4, Solution::island_perimeter(vec![vec![1]]));
    }

    #[test]
    fn test_3() {
        assert_eq!(4, Solution::island_perimeter(vec![vec![1, 0]]));
    }
}
