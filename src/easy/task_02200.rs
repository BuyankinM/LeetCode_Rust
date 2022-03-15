// 2200. Find All K-Distant Indices in an Array
// https://leetcode.com/problems/find-all-k-distant-indices-in-an-array/

use crate::Solution;

impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let l = (nums.len() - 1) as i32;
        let mut res = Vec::with_capacity(nums.len());
        let mut start;
        let mut end = -1;

        for (_, idx) in nums.iter().zip(0..).filter(|(&x, _)| x == key) {
            start = (end + 1).max(idx - k);
            end = l.min(idx + k);

            (start..=end).into_iter().for_each(|x| res.push(x));
            if end == l {
                break;
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
            vec![0, 1, 2, 3, 4],
            Solution::find_k_distant_indices(vec![2, 2, 2, 2, 2], 2, 2)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6],
            Solution::find_k_distant_indices(vec![3, 4, 9, 1, 3, 9, 5], 9, 1)
        );
    }
}
