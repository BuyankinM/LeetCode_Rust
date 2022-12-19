//417. Pacific Atlantic Water Flow
//https://leetcode.com/problems/pacific-atlantic-water-flow/
use crate::Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let rows = matrix.len();
        let cols = matrix.first().map_or(0, |v| v.len());

        if rows == 0 {
            return res;
        }

        let mut path = (0..rows).map(|_| vec![0u8; cols]).collect::<Vec<Vec<u8>>>();
        let mut points = VecDeque::new();

        (0..rows).for_each(|r| {
            points.push_back((r as i32, 0, 1));
            path[r][0] = 1;

            points.push_back((r as i32, (cols - 1) as i32, 2));
            path[r][cols - 1] = 2;
        });

        (0..cols).for_each(|c| {
            points.push_back((0, c as i32, 1));
            path[0][c] = 1;

            points.push_back(((rows - 1) as i32, c as i32, 2));
            path[rows - 1][c] = 2;
        });

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some((x, y, ocean)) = points.pop_front() {
            path[x as usize][y as usize] |= ocean;

            for &(xd, yd) in directions.iter() {
                let (x_new, y_new) = (x + xd, y + yd);

                // check out of bounds
                if x_new < 0 || x_new > (rows - 1) as i32 || y_new < 0 || y_new > (cols - 1) as i32
                {
                    continue;
                }

                // check visited
                let new_ocean = path[x_new as usize][y_new as usize];
                if new_ocean == ocean || new_ocean == 3 {
                    continue;
                }

                // check heights
                if matrix[x_new as usize][y_new as usize] < matrix[x as usize][y as usize] {
                    continue;
                }

                points.push_back((x_new, y_new, ocean));
            }
        }

        for (r, row) in path.iter().enumerate() {
            for (c, val) in row.iter().enumerate() {
                if *val == 3 {
                    res.push(vec![r as i32, c as i32]);
                }
            }
        }
        res
    }
}
