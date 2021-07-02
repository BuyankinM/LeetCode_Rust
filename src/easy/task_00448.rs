// 448. Find All Numbers Disappeared in an Array
// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/

use crate::Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![false; nums.len() + 1];
        nums.iter().for_each(|&x| {
            res[x as usize] = true;
        });
        res.iter()
            .enumerate()
            .skip(1)
            .filter_map(|(i, &x)| if !x { Some(i as i32) } else { None })
            .collect()
    }

    pub fn find_disappeared_numbers_sign(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for n in 0..nums.len() {
            let idx = (nums[n].abs() - 1) as usize;
            nums[idx] = -nums[idx].abs();
        }

        nums.iter()
            .enumerate()
            .filter_map(|(ind, n)| if *n > 0 { Some(ind as i32 + 1) } else { None })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![5, 6],
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![2], Solution::find_disappeared_numbers(vec![1, 1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![2, 3],
            Solution::find_disappeared_numbers(vec![1, 1, 1])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(vec![2], Solution::find_disappeared_numbers_sign(vec![1, 1]));
    }

    #[test]
    fn test_5() {
        assert_eq!(
            vec![2, 3],
            Solution::find_disappeared_numbers_sign(vec![1, 1, 1])
        );
    }
}
