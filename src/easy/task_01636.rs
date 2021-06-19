// 1636. Sort Array by Increasing Frequency
// https://leetcode.com/problems/sort-array-by-increasing-frequency/

use crate::Solution;

impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let mut h = std::collections::HashMap::new();
        nums.iter().for_each(|&x| *(h.entry(x).or_insert(0)) += 1);
        nums.sort_unstable_by_key(|x| (h[x], -(*x)));
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![3, 1, 1, 2, 2, 2],
            Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![1, 3, 3, 2, 2],
            Solution::frequency_sort(vec![2, 3, 1, 3, 2])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![5, -1, 4, 4, -6, -6, 1, 1, 1],
            Solution::frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1])
        );
    }
}
