// 532. K-diff Pairs in an Array
// https://leetcode.com/problems/k-diff-pairs-in-an-array/

use crate::Solution;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = std::collections::HashMap::new();
        nums.iter().for_each(|&x| *map.entry(x).or_insert(0) += 1);

        let mut counter = 0;
        map.iter().for_each(|(&x, &n)| {
            if map.get(&(x + k)).is_some() {
                counter += (k > 0 || k == 0 && n > 1) as i32
            }
        });
        counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::find_pairs(vec![3, 1, 4, 1, 5], 2));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, Solution::find_pairs(vec![1, 2, 3, 4, 5], 1));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::find_pairs(vec![1, 3, 1, 5, 4], 0));
    }
}
