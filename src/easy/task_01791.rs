// 1791. Find Center of Star Graph
// https://leetcode.com/problems/find-center-of-star-graph/

use crate::Solution;

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let (u0, v0) = (edges[0][0], edges[0][1]);
        let (u1, v1) = (edges[1][0], edges[1][1]);

        match u0 == u1 || u0 == v1 {
            true => u0,
            false => v0,
        }
    }

    pub fn find_center_short(edges: Vec<Vec<i32>>) -> i32 {
        if edges[0][0] == edges[1][0] || edges[0][0] == edges[1][1] {
            edges[0][0]
        } else {
            edges[0][1]
        }
    }

    pub fn find_center_unsafe(edges: Vec<Vec<i32>>) -> i32 {
        let u0;
        let v0;
        let u1;
        let v1;

        unsafe {
            u0 = *edges.get_unchecked(0).get_unchecked(0);
            v0 = *edges.get_unchecked(0).get_unchecked(1);

            u1 = *edges.get_unchecked(1).get_unchecked(0);
            v1 = *edges.get_unchecked(1).get_unchecked(1);
        }

        match u0 == u1 || u0 == v1 {
            true => u0,
            false => v0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            1,
            Solution::find_center(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]])
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            2,
            Solution::find_center_short(vec![vec![1, 2], vec![2, 3], vec![4, 2]])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            1,
            Solution::find_center_unsafe(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]])
        );
    }
}
