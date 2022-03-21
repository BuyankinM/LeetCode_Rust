// 2210. Count Hills and Valleys in an Array
// https://leetcode.com/problems/count-hills-and-valleys-in-an-array/

use crate::Solution;

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let mut cur_sign;
        let mut count = 0;
        let mut prev_sign = 0;
        for (&x, &y) in nums.iter().zip(nums[1..].iter()).filter(|(x, y)| x != y) {
            cur_sign = (y - x).signum();
            if cur_sign != prev_sign && prev_sign != 0 {
                count += 1;
            }
            prev_sign = cur_sign;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::count_hill_valley(vec![2, 4, 1, 1, 6, 5]));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::count_hill_valley(vec![6, 6, 5, 5, 4, 1]));
    }
}
