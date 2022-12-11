// 2500. Delete Greatest Value in Each Row
// https://leetcode.com/problems/delete-greatest-value-in-each-row/

use crate::Solution;

impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        grid.iter_mut()
            .for_each(|row| row.sort_unstable_by_key(|&x| -x));
        (0..grid[0].len())
            .map(|i| grid.iter().map(|row| row[i]).max().unwrap())
            .sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            8,
            Solution::delete_greatest_value(vec![vec![1, 2, 4], vec![3, 3, 1]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(10, Solution::delete_greatest_value(vec![vec![10]]));
    }
}
