// 349. Intersection of Two Arrays
// https://leetcode.com/problems/intersection-of-two-arrays/

use crate::Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut counter = [false; 1001];
        nums1.iter().for_each(|&x| counter[x as usize] = true);
        nums2
            .iter()
            .filter(|&&x| match counter[x as usize] {
                true => {
                    counter[x as usize] = false;
                    true
                }
                false => false,
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            vec![9, 4]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2, 1]),
            vec![2, 1]
        );
    }
}
