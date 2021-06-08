// 1470. Shuffle the Array
// https://leetcode.com/problems/shuffle-the-array/

use crate::Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut v = vec![0; nums.len()];
        let mid = n as usize;

        for i in 0..mid {
            v[2 * i] = nums[i];
            v[2 * i + 1] = nums[i + mid];
        }

        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![2,3,5,4,1,7], Solution::shuffle(vec![2,5,1,3,4,7], 3));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![1,4,2,3,3,2,4,1], Solution::shuffle(vec![1,2,3,4,4,3,2,1], 4));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![1,2,1,2], Solution::shuffle(vec![1,1,2,2], 2));
    }
}
