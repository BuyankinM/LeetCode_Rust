// 2389. Longest Subsequence With Limited Sum
// https://leetcode.com/problems/longest-subsequence-with-limited-sum/

use crate::Solution;

impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut prev = 0;
        let mut sums = nums;
        sums.sort_unstable();
        sums.iter_mut().for_each(|x| {
            *x += prev;
            prev = *x;
        });

        queries
            .iter()
            .map(|q| match sums.binary_search(q) {
                Ok(i) => i + 1,
                Err(i) => i,
            } as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21]),
            vec![2, 3, 4]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::answer_queries(vec![2, 3, 4, 5], vec![1]), vec![0]);
    }
}
