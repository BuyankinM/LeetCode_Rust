// 797. All Paths From Source to Target
// https://leetcode.com/problems/all-paths-from-source-to-target/description/

use crate::Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(
            cur_node: i32,
            end_node: i32,
            mut cur_path: Vec<i32>,
            graph: &[Vec<i32>],
            res: &mut Vec<Vec<i32>>,
        ) {
            cur_path.push(cur_node);

            match cur_node < end_node {
                true => graph[cur_node as usize]
                    .iter()
                    .for_each(|&node| dfs(node, end_node, cur_path.clone(), graph, res)),
                false => res.push(cur_path),
            }
        }

        let end_node = (graph.len() - 1) as i32;
        let cur_path = Vec::new();
        let mut res = Vec::new();

        dfs(0, end_node, cur_path, &graph, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![vec![0, 1, 3], vec![0, 2, 3]],
            Solution::all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![
                vec![0, 4],
                vec![0, 3, 4],
                vec![0, 1, 3, 4],
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 4]
            ],
            Solution::all_paths_source_target(vec![
                vec![4, 3, 1],
                vec![3, 2, 4],
                vec![3],
                vec![4],
                vec![]
            ])
        );
    }
}
