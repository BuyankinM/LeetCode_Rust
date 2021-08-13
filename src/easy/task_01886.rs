// 1886. Determine Whether Matrix Can Be Obtained By Rotation
// https://leetcode.com/problems/determine-whether-matrix-can-be-obtained-by-rotation/

use crate::Solution;

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let n = mat.len() - 1;
        let mut arr_counter_rotate = [0; 4];

        mat.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &val)| {
                arr_counter_rotate[0] += (val == target[i][j]) as usize;
                arr_counter_rotate[1] += (val == target[j][n - i]) as usize;
                arr_counter_rotate[2] += (val == target[n - i][n - j]) as usize;
                arr_counter_rotate[3] += (val == target[n - j][i]) as usize;
            })
        });
        arr_counter_rotate.iter().any(|x| *x == (n + 1).pow(2))
    }

    pub fn find_rotation_flags(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let n = mat.len() - 1;
        let mut rotations = [true; 4];

        mat.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &val)| {
                rotations[0] &= val == target[i][j];
                rotations[1] &= val == target[j][n - i];
                rotations[2] &= val == target[n - i][n - j];
                rotations[3] &= val == target[n - j][i];
            })
        });
        rotations.iter().any(|x| *x)
    }

    pub fn find_rotation_functional(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let n = mat.len() - 1;
        let mut rotations = [true; 4];

        mat.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &val)| {
                rotations
                    .iter_mut()
                    .enumerate()
                    .filter(|(_, flag)| **flag)
                    .for_each(|(rot, flag)| {
                        *flag = match rot {
                            0 => val == target[i][j],
                            1 => val == target[j][n - i],
                            2 => val == target[n - i][n - j],
                            _ => val == target[n - j][i],
                        }
                    });
            })
        });
        rotations.iter().any(|x| *x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::find_rotation(
            vec![vec![0, 1], vec![1, 0]],
            vec![vec![1, 0], vec![0, 1]]
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::find_rotation_flags(
            vec![vec![0, 1], vec![1, 1]],
            vec![vec![1, 0], vec![0, 1]]
        ));
    }

    #[test]
    fn test_3() {
        assert!(Solution::find_rotation_functional(
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
            vec![vec![1, 1, 1], vec![0, 1, 0], vec![0, 0, 0]]
        ));
    }
}
