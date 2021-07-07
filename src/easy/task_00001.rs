// 1. Two Sum
// https://leetcode.com/problems/two-sum/

use crate::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n: usize = nums.len();
        for i in 0..(n - 1) {
            let elem = target - nums[i];
            for j in (i + 1)..n {
                if nums[j] == elem {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }

    pub fn two_sum_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut m = HashMap::with_capacity(nums.len());
        for (i, v) in nums.iter().enumerate() {
            match m.get(&(target - *v)) {
                Some(&i2) => return vec![i as i32, i2],
                None => m.insert(*v, i as i32),
            };
        }
        vec![]
    }

    pub fn two_sum_two_pointers(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp::Ordering;

        let mut sorted: Vec<_> = nums.iter().enumerate().map(|(ind, x)| (x, ind)).collect();
        sorted.sort_unstable();

        let mut start = sorted.iter().peekable();
        let mut end = sorted.iter().rev().peekable();

        while let (Some(&&(low, i)), Some(&&(high, j))) = (start.peek(), end.peek()) {
            match (low + high).cmp(&target) {
                Ordering::Less => start.next(),
                Ordering::Equal => return vec![i as i32, j as i32],
                Ordering::Greater => end.next(),
            };
        }
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![2, 1], Solution::two_sum_hashmap(vec![3, 2, 4], 6));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![0, 1], Solution::two_sum_two_pointers(vec![3, 3], 6));
    }
}
