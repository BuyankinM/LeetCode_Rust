// 1389. Create Target Array in the Given Order
// https://leetcode.com/problems/create-target-array-in-the-given-order/

use crate::Solution;

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(nums.len());
        nums.iter()
            .zip(index.iter())
            .for_each(|(x, ind)| res.insert(*ind as usize, *x));
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![0, 4, 1, 3, 2],
            Solution::create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![0, 1, 2, 3, 4],
            Solution::create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![1], Solution::create_target_array(vec![1], vec![0]));
    }
}
