// 892. Surface Area of 3D Shapes
// https://leetcode.com/problems/surface-area-of-3d-shapes/

use crate::Solution;

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let l = grid.len() as i32;
        let mut res = 0;

        for (row, i) in grid.iter().zip(0..) {
            for (&val, j) in row.iter().zip(0..).filter(|(val, _)| **val > 0) {
                res += 2; // up adn down surfaces
                for (dx, dy) in dirs.iter().cloned() {
                    let (xx, yy) = (j + dx, i + dy);
                    res += match xx >= 0 && yy >= 0 && xx < l && yy < l {
                        true => (val - grid[yy as usize][xx as usize]).max(0),
                        false => val,
                    };
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(10, Solution::surface_area(vec![vec![2]]));
    }

    #[test]
    fn test_2() {
        assert_eq!(34, Solution::surface_area(vec![vec![1, 2], vec![3, 4]]));
    }

    #[test]
    fn test_3() {
        assert_eq!(16, Solution::surface_area(vec![vec![1, 0], vec![0, 2]]));
    }

    #[test]
    fn test_4() {
        assert_eq!(
            32,
            Solution::surface_area(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]])
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            46,
            Solution::surface_area(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]])
        );
    }
}
