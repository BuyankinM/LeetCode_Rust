// 2206. Divide Array Into Equal Pairs
// https://leetcode.com/problems/divide-array-into-equal-pairs/

use crate::Solution;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut counter = [0; 501];
        nums.iter().for_each(|&x| counter[x as usize] += 1);
        counter.iter().filter(|x| **x > 0).all(|&x| x & 1 == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(!Solution::divide_array(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_2() {
        assert!(Solution::divide_array(vec![3, 2, 3, 2, 2, 2]));
    }
}
