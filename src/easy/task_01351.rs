// 1351. Count Negative Numbers in a Sorted Matrix
// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/

use crate::Solution;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        fn neg_binsearch(v: &Vec<i32>, mut high: i32) -> i32 {
            let mut low = 0;
            while low < high {
                let mid = low + (high - low) / 2;
                match v[mid as usize] >= 0 {
                    true => low = mid + 1,
                    false => high = mid,
                }
            }
            high
        }

        let (rows, cols) = (grid.len(), grid[0].len() as i32);
        let mut high = cols;
        let mut res = 0;

        for (i, row) in grid.iter().enumerate() {
            let new_high = neg_binsearch(row, high);
            if new_high < cols {
                res += (high - new_high) * (rows - i) as i32; // calc square of rectangle between new and last find indicies
                if new_high == 0 {
                    break;
                }
            }
            high = new_high;
        }
        res
    }

    pub fn count_negatives_one_line_1(grid: Vec<Vec<i32>>) -> i32 {
        grid.iter()
            .map(|v| v.iter().filter(|x| **x < 0).sum::<i32>())
            .sum::<i32>()
    }

    pub fn count_negatives_one_line_2(grid: Vec<Vec<i32>>) -> i32 {
        grid.into_iter().flatten().filter(|&x| x < 0).count() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            8,
            Solution::count_negatives(vec![
                vec![4, 3, 2, -1],
                vec![3, 2, 1, -1],
                vec![1, 1, -1, -2],
                vec![-1, -1, -2, -3]
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            0,
            Solution::count_negatives_one_line_1(vec![vec![3, 2], vec![1, 0]])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::count_negatives_one_line_2(vec![vec![-1]]));
    }
}
