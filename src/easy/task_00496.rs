// 496. Next Greater Element I
// https://leetcode.com/problems/next-greater-element-i/

use crate::Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut res = Vec::with_capacity(nums1.len());
        let mut map = HashMap::with_capacity(nums2.len());
        let mut stack: Vec<i32> = Vec::with_capacity(nums2.len());

        for &val2 in &nums2 {
            while !stack.is_empty() && *stack.last().unwrap() < val2 {
                map.insert(stack.pop().unwrap(), val2);
            }
            stack.push(val2);
        }

        for val1 in &nums1 {
            res.push(*map.get(val1).unwrap_or(&-1));
        }

        res
    }

    pub fn next_greater_element_slow(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(nums1.len());

        'outer: for &val1 in &nums1 {
            let mut find = false;
            for &val2 in &nums2 {
                if val2 == val1 {
                    find = true
                }
                if find && val2 > val1 {
                    res.push(val2);
                    continue 'outer;
                }
            }
            res.push(-1);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![-1, 3, -1],
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![3, -1],
            Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4])
        );
    }
}
