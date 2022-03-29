// 287. Find the Duplicate Number
// https://leetcode.com/problems/find-the-duplicate-number/

use crate::Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let (mut tortoise, mut hare) = (nums[0], nums[0]);
        loop {
            tortoise = nums[tortoise as usize];
            hare = nums[nums[hare as usize] as usize];
            if tortoise == hare {
                break;
            }
        }

        tortoise = nums[0];

        while tortoise != hare {
            tortoise = nums[tortoise as usize];
            hare = nums[hare as usize];
        }

        hare
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::find_duplicate(vec![1, 3, 4, 2, 2]));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::find_duplicate(vec![3, 1, 3, 4, 2]));
    }
}
