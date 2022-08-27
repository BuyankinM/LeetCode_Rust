// 2368. Reachable Nodes With Restrictions
// https://leetcode.com/problems/reachable-nodes-with-restrictions/

use crate::Solution;

impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        use std::collections::{HashMap, HashSet};

        let mut graph: HashMap<i32, Vec<i32>> = HashMap::with_capacity(n as usize);
        let mut visited = restricted.into_iter().collect::<HashSet<_>>();
        edges.iter().for_each(|v| {
            if let &[n1, n2] = v.as_slice() {
                if !visited.contains(&n1) && !visited.contains(&n2) {
                    graph
                        .entry(n1)
                        .and_modify(|v| v.push(n2))
                        .or_insert_with(|| vec![n2]);

                    graph
                        .entry(n2)
                        .and_modify(|v| v.push(n1))
                        .or_insert_with(|| vec![n1]);
                }
            }
        });

        let mut res = 1;
        let mut stack = vec![0];
        visited.insert(0);

        while let Some(n) = stack.pop() {
            if let Some(v) = graph.get(&n) {
                for &e in v.iter() {
                    if visited.insert(e) {
                        stack.push(e);
                        res += 1;
                    }
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
        assert_eq!(
            Solution::reachable_nodes(
                7,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![3, 1],
                    vec![4, 0],
                    vec![0, 5],
                    vec![5, 6]
                ],
                vec![4, 5]
            ),
            4
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::reachable_nodes(
                7,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 5],
                    vec![0, 4],
                    vec![3, 2],
                    vec![6, 5]
                ],
                vec![4, 2, 1]
            ),
            3
        );
    }
}
