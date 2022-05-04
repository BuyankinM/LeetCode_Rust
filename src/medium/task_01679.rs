// 1679. Max Number of K-Sum Pairs
// https://leetcode.com/problems/max-number-of-k-sum-pairs/

use crate::Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut map = std::collections::HashMap::with_capacity(nums.len());
        nums.iter().for_each(|&x| {
            let e = map.entry(x).or_insert(0);
            match *e {
                0 => *map.entry(k - x).or_insert(0) += 1,
                _ => {
                    *e -= 1;
                    count += 1;
                }
            }
        });
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_operations(vec![1, 2, 3, 4], 5), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_operations(vec![3, 1, 3, 4, 3], 6), 1);
    }
}
