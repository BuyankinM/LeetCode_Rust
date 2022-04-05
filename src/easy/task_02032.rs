// 2032. Two Out of Three
// https://leetcode.com/problems/two-out-of-three/

use crate::Solution;

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut counter = [0_i32; 101];
        nums1.iter().for_each(|&x| counter[x as usize] |= 0b001);
        nums2.iter().for_each(|&x| counter[x as usize] |= 0b010);
        nums3.iter().for_each(|&x| counter[x as usize] |= 0b100);
        counter
            .iter()
            .enumerate()
            .filter_map(|(ind, x)| match x.count_ones() >= 2 {
                true => Some(ind as i32),
                false => None,
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![2, 3],
            Solution::two_out_of_three(vec![1, 1, 3, 2], vec![2, 3], vec![3])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![1, 2, 3],
            Solution::two_out_of_three(vec![3, 1], vec![2, 3], vec![1, 2])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![0_i32; 0],
            Solution::two_out_of_three(vec![1, 2, 2], vec![4, 3, 3], vec![5])
        );
    }
}
