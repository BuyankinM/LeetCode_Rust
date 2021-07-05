// 1920. Build Array from Permutation
// https://leetcode.com/problems/build-array-from-permutation/

use crate::Solution;

impl Solution {
    pub fn build_array_permutations(nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len()).map(|i| nums[nums[i] as usize]).collect()
    }

    pub fn build_array_permutations_bits_O1(mut nums: Vec<i32>) -> Vec<i32> {
        let mask = 1023;
        (0..nums.len()).for_each(|i| nums[i] |= (nums[nums[i] as usize] & mask) << 10);
        (0..nums.len()).for_each(|i| nums[i] >>= 10);
        nums
    }

    pub fn build_array_permutations_mod_O1(mut nums: Vec<i32>) -> Vec<i32> {
        let l = nums.len() as i32;
        (0..l as usize).for_each(|i| nums[i] += (nums[nums[i] as usize] % l) * l);
        (0..l as usize).for_each(|i| nums[i] /= l);
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![0, 1, 2, 4, 5, 3],
            Solution::build_array_permutations(vec![0, 2, 1, 5, 3, 4])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![4, 5, 0, 1, 2, 3],
            Solution::build_array_permutations(vec![5, 0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![4, 5, 0, 1, 2, 3],
            Solution::build_array_permutations_bits_O1(vec![5, 0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            vec![4, 5, 0, 1, 2, 3],
            Solution::build_array_permutations_mod_O1(vec![5, 0, 1, 2, 3, 4])
        );
    }
}
