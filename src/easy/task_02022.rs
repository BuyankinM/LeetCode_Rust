// 2022. Convert 1D Array Into 2D Array
// https://leetcode.com/problems/convert-1d-array-into-2d-array/

use crate::Solution;

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        match original.len() as i32 == m * n {
            false => vec![],
            true => original
                .chunks(n as usize)
                .map(|v| v.to_vec())
                .collect::<Vec<_>>(),
        }
    }

    pub fn construct2_d_array_iter(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let (m, n) = (m as usize, n as usize);
        if original.len() != m * n {
            return Vec::new();
        }
        let mut original = original.into_iter();
        (0..m)
            .map(|_| original.by_ref().take(n).collect())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![vec![1, 2], vec![3, 4]],
            Solution::construct2_d_array(vec![1, 2, 3, 4], 2, 2)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![vec![1, 2, 3]],
            Solution::construct2_d_array(vec![1, 2, 3], 1, 3)
        );
    }

    #[test]
    fn test_3() {
        let v: Vec<Vec<i32>> = vec![];
        assert_eq!(v, Solution::construct2_d_array(vec![1, 2], 1, 1));
    }

    #[test]
    fn test_4() {
        let v: Vec<Vec<i32>> = vec![];
        assert_eq!(v, Solution::construct2_d_array(vec![3], 1, 2));
    }
}
