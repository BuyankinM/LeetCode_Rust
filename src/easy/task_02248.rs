// 2248. Intersection of Multiple Arrays
// https://leetcode.com/problems/intersection-of-multiple-arrays/

use crate::Solution;

impl Solution {
    pub fn intersection_2248(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let l = nums.len();
        let mut counter = [0; 1001];
        nums.iter()
            .flat_map(|v| v.iter())
            .for_each(|v| counter[*v as usize] += 1);

        counter
            .iter()
            .zip(0..)
            .filter_map(|(&num, idx)| if num == l { Some(idx) } else { None })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::intersection_2248(vec![
                vec![3, 1, 2, 4, 5],
                vec![1, 2, 3, 4],
                vec![3, 4, 5, 6]
            ]),
            vec![3, 4]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::intersection_2248(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            vec![]
        );
    }
}
