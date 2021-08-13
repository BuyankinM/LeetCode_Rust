// 1313. Decompress Run-Length Encoded List
// https://leetcode.com/problems/decompress-run-length-encoded-list/

use crate::Solution;

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        nums.chunks_exact(2)
            .flat_map(|v| std::iter::repeat(v[1]).take(v[0] as usize))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![2, 4, 4, 4],
            Solution::decompress_rl_elist(vec![1, 2, 3, 4])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![1, 3, 3],
            Solution::decompress_rl_elist(vec![1, 1, 2, 3])
        );
    }
}
