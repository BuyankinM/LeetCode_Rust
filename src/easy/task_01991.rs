// 1991. Find the Middle Index in Array
// https://leetcode.com/problems/find-the-middle-index-in-array/

use crate::Solution;

impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let total_sum = nums.iter().sum::<i32>();
        let mut prev_sum = 0;

        for (ind, &num) in nums.iter().enumerate() {
            match total_sum - prev_sum - num == prev_sum {
                true => return ind as i32,
                false => prev_sum += num,
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::find_middle_index(vec![2, 3, -1, 8, 4]));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::find_middle_index(vec![1, -1, 4]));
    }

    #[test]
    fn test_3() {
        assert_eq!(-1, Solution::find_middle_index(vec![2, 5]));
    }

    #[test]
    fn test_4() {
        assert_eq!(0, Solution::find_middle_index(vec![1]));
    }
}
