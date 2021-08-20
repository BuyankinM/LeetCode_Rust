// 1971. Find if Path Exists in Graph
// https://leetcode.com/problems/find-if-path-exists-in-graph/

use crate::Solution;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, start: i32, end: i32) -> bool {
        use std::collections::{HashMap, HashSet};

        if start == end || edges.is_empty() {
            return true;
        }

        let mut edges_map = HashMap::with_capacity(n as usize);
        edges.iter().for_each(|v| {
            edges_map.entry(v[0]).or_insert_with(Vec::new).push(v[1]);
            edges_map.entry(v[1]).or_insert_with(Vec::new).push(v[0]);
        });

        let mut set = HashSet::with_capacity(n as usize);
        let mut stack = edges_map[&start].clone();
        while let Some(val) = stack.pop() {
            if !set.insert(val) {
                continue;
            };

            for &node in edges_map[&val].iter() {
                match node == end {
                    true => return true,
                    false => stack.push(node),
                }
            }
        }
        false
    }

    pub fn valid_path_unionsort(n: i32, edges: Vec<Vec<i32>>, start: i32, end: i32) -> bool {
        fn find(v: &[i32], i: i32) -> i32 {
            match v[i as usize] {
                val if val < 0 => i,
                val => find(v, val),
            }
        }

        let mut v = vec![-1; n as usize];
        edges.iter().for_each(|edge| {
            let p1 = find(&v, edge[0]);
            let p2 = find(&v, edge[1]);
            if p1 != p2 {
                v[p2 as usize] = p1;
            }
        });

        find(&v, start) == find(&v, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::valid_path(
            3,
            vec![vec![0, 1], vec![1, 2], vec![2, 0]],
            0,
            2
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::valid_path(
            6,
            vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]],
            0,
            5
        ));
    }

    #[test]
    fn test_3() {
        assert!(Solution::valid_path(1, vec![], 0, 0));
    }

    #[test]
    fn test_4() {
        assert!(Solution::valid_path_unionsort(
            3,
            vec![vec![0, 1], vec![1, 2], vec![2, 0]],
            0,
            2
        ));
    }
}
