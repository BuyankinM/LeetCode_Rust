// 1365. How Many Numbers Are Smaller Than the Current Number
// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/

use crate::Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        fn binary_search_min(nums: &Vec<i32>, target: i32) -> i32 {
            let (mut low, mut high) = (0i32, nums.len() as i32);
            while low <= high {
                let mid = low + (high - low) / 2;
                match nums[mid as usize] >= target {
                    true => high = mid - 1,
                    false => low = mid + 1,
                };
            }
            high + 1
        }

        let mut sort_nums = nums.clone();
        sort_nums.sort_unstable();

        nums.iter()
            .map(|&x| binary_search_min(&sort_nums, x))
            .collect()
    }

    pub fn smaller_numbers_than_current_hashmap(nums: Vec<i32>) -> Vec<i32> {
        let mut hm = std::collections::HashMap::new();

        let mut sorted = nums.clone();
        sorted.sort_unstable();
        sorted.iter().enumerate().for_each(|(i, n)| {
            hm.entry(n).or_insert(i);
        });

        nums.iter().map(|n| hm[n] as i32).collect()
    }

    pub fn smaller_numbers_than_current_running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut arr_sum = [0; 101];
        nums.iter().for_each(|x| arr_sum[*x as usize] += 1);
        (1..101).for_each(|i| arr_sum[i] += arr_sum[i - 1]);
        nums.iter()
            .map(|&x| if x > 0 { arr_sum[(x - 1) as usize] } else { 0 })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![4, 0, 1, 1, 3],
            Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![2, 1, 0, 3],
            Solution::smaller_numbers_than_current(vec![6, 5, 4, 8])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![0, 0, 0, 0],
            Solution::smaller_numbers_than_current(vec![7, 7, 7, 7])
        );
    }
}
