// 733. Flood Fill
// https://leetcode.com/problems/flood-fill/

use crate::Solution;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let (rows, cols) = (image.len() as i32, image[0].len() as i32);
        let old_color = image[sr as usize][sc as usize];
        let mut stack = vec![(sr, sc)];
        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut visited = std::collections::HashSet::new();

        while let Some((x, y)) = stack.pop() {
            if !visited.insert((x, y)) {
                continue;
            }

            image[x as usize][y as usize] = new_color;

            for &(dx, dy) in dirs.iter() {
                let new_x = x + dx;
                let new_y = y + dy;
                if new_x >= 0
                    && new_x < rows
                    && new_y >= 0
                    && new_y < cols
                    && image[new_x as usize][new_y as usize] == old_color
                {
                    stack.push((new_x, new_y));
                }
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]],
            Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![vec![2, 2, 2], vec![2, 2, 2]],
            Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 2)
        );
    }
}
