// 581. Shortest Unsorted Continuous Subarray
// https://leetcode.com/problems/shortest-unsorted-continuous-subarray/

use crate::Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut v = nums.clone();
        v.sort_unstable();

        let (mut left, mut right) = (0i32, v.len() as i32 - 1);
        while left < v.len() as i32 && v[left as usize] == nums[left as usize] {
            left += 1;
        }
        while right > left && v[right as usize] == nums[right as usize] {
            right -= 1;
        }
        right - left + 1
    }

    // aka_npou
    pub fn find_unsorted_subarray_opt(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }

        let (mut head, mut tail) = (-2, -2);
        let mut max = nums[0];

        for ((&cur, &next), i) in nums.iter().zip(nums.iter().skip(1)).zip(0..) {
            if cur <= next && next >= max {
                max = next;
            } else {
                if head == -2 {
                    head = i;
                }

                while head >= 0 && nums[head as usize] > next {
                    head -= 1;
                }

                tail = i + 1;
            }
        }

        tail - head
    }

    // https://leetcode.com/problems/shortest-unsorted-continuous-subarray/discuss/2003972/easy-Rust-5-lines
    pub fn find_unsorted_subarray_func(nums: Vec<i32>) -> i32 {
        let (mut max, mut min) = (i32::MIN, i32::MAX);
        nums.iter().fold(i32::MIN, |prev, next| {
            if prev > *next {
                max = max.max(prev);
                min = min.min(*next);
            }
            *next
        });
        let l = nums.iter().position(|&v| v > min).unwrap_or(0);
        let r = nums.len()
            - nums
                .iter()
                .rev()
                .position(|&v| v < max)
                .unwrap_or(nums.len());
        (r - l) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]),
            5
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_unsorted_subarray(vec![1, 2, 3, 4]), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::find_unsorted_subarray(vec![1]), 0);
    }
}
