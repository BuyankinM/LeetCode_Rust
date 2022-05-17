// 1091. Shortest Path in Binary Matrix
// https://leetcode.com/problems/shortest-path-in-binary-matrix/

use crate::Solution;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    dist: i32,
}

impl Point {
    fn new(x: usize, y: usize, dist: i32) -> Self {
        Self { x, y, dist }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let n_i = n as i32;
        if grid[0][0] == 1 || grid[n - 1][n - 1] == 1 {
            return -1;
        }

        let mut map = vec![vec![-1; n]; n];
        let mut visited = vec![vec![false; n]; n];
        let mut heap = BinaryHeap::new();
        heap.push(Point::new(0, 0, 1));

        let dirs = [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];

        while let Some(p) = heap.pop() {
            let Point { x, y, dist } = p;

            if visited[x][y] || grid[x][y] == 1 {
                continue;
            } else if x == n - 1 && y == n - 1 {
                return dist;
            }

            visited[x][y] = true;

            for &(dx, dy) in &dirs {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                let new_dist = dist + 1;
                if nx < 0 || nx >= n_i || ny < 0 || ny >= n_i {
                    continue;
                }
                if map[nx as usize][ny as usize] == -1 {
                    map[nx as usize][ny as usize] = new_dist;
                    heap.push(Point::new(nx as usize, ny as usize, new_dist));
                }
            }
        }
        map[n - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![vec![0, 1], vec![1, 0]]),
            2
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![0, 0, 0],
                vec![1, 1, 0],
                vec![1, 1, 0]
            ]),
            4
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![1, 0, 0],
                vec![1, 1, 0],
                vec![1, 1, 0]
            ]),
            -1
        );
    }
}
