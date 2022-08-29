// 200. Number of Islands
// https://leetcode.com/problems/number-of-islands/

use crate::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut visited = HashSet::new();
        let mut res = 0;
        let size = (grid.len() as i32, grid[0].len() as i32);

        for (row, i) in grid.iter().zip(0..) {
            for (_, j) in row.iter().zip(0..).filter(|(&c, _)| c == '1') {
                if visited.insert((i, j)) {
                    Solution::dfs_map(&grid, &mut visited, (i, j), size);
                    res += 1;
                }
            }
        }
        res
    }

    fn dfs_map(
        grid: &[Vec<char>],
        visited: &mut HashSet<(i32, i32)>,
        pos: (i32, i32),
        size: (i32, i32),
    ) {
        let mut stack = vec![pos];
        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        while let Some(pos) = stack.pop() {
            for &(di, dj) in &dirs {
                let new_i = pos.0 + di;
                let new_j = pos.1 + dj;
                if new_i >= 0
                    && new_j >= 0
                    && new_i < size.0
                    && new_j < size.1
                    && grid[new_i as usize][new_j as usize] == '1'
                    && visited.insert((new_i, new_j))
                {
                    stack.push((new_i, new_j))
                }
            }
        }
    }

    // https://leetcode.com/problems/number-of-islands/discuss/2105704/Rust-or-Iterative-DFS-%2B-BFS-or-With-Comments
    pub fn num_islands_short(mut grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        let mut stack = vec![];
        let (m, n) = (grid.len(), grid[0].len());

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    stack.push((i, j));
                    while let Some((r, c)) = stack.pop() {
                        if r < m && c < n && grid[r][c] == '1' {
                            grid[r][c] = '0';
                            for rc in [0, 1, 0, !0, 0].windows(2) {
                                stack.push((r.wrapping_add(rc[0]), c.wrapping_add(rc[1])));
                            }
                        }
                    }
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            1
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            3
        );
    }
}
