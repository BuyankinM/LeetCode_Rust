// 406. Queue Reconstruction by Height
// https://leetcode.com/problems/queue-reconstruction-by-height/

use crate::Solution;

impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = Vec::with_capacity(people.len());
        people.sort_unstable_by_key(|v| (-v[0], v[1]));
        people
            .into_iter()
            .for_each(|v| res.insert(v[1] as usize, v));
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::reconstruct_queue(vec![
                vec![7, 0],
                vec![4, 4],
                vec![7, 1],
                vec![5, 0],
                vec![6, 1],
                vec![5, 2]
            ]),
            vec![
                vec![5, 0],
                vec![7, 0],
                vec![5, 2],
                vec![6, 1],
                vec![4, 4],
                vec![7, 1]
            ]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::reconstruct_queue(vec![
                vec![6, 0],
                vec![5, 0],
                vec![4, 0],
                vec![3, 2],
                vec![2, 2],
                vec![1, 4]
            ]),
            vec![
                vec![4, 0],
                vec![5, 0],
                vec![2, 2],
                vec![3, 2],
                vec![1, 4],
                vec![6, 0]
            ]
        );
    }
}
