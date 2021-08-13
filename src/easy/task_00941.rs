// 941. Valid Mountain Array
// https://leetcode.com/problems/valid-mountain-array/

use crate::Solution;

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() <= 2 || arr[0] >= arr[1] {
            return false;
        }

        let mut h = arr[0];
        let mut up = true;

        for &val in &arr[1..] {
            if (!up && h <= val) || (up && h == val) {
                return false;
            }
            if up && h > val {
                up = false
            }
            h = val;
        }
        !up
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(!Solution::valid_mountain_array(vec![2, 1]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::valid_mountain_array(vec![3, 5, 5]));
    }

    #[test]
    fn test_3() {
        assert!(Solution::valid_mountain_array(vec![0, 3, 2, 1]));
    }
}
