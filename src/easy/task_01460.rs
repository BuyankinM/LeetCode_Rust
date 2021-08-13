// 1460. Make Two Arrays Equal by Reversing Sub-arrays
// https://leetcode.com/problems/make-two-arrays-equal-by-reversing-sub-arrays/

use crate::Solution;

impl Solution {
    pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
        target.sort_unstable();
        arr.sort_unstable();
        target == arr
    }

    pub fn can_be_equal_optimal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        !target
            .iter()
            .zip(arr.iter())
            .fold([0; 1001], |mut acc, (a, b)| {
                acc[*a as usize] += 1;
                acc[*b as usize] -= 1;
                acc
            })
            .iter()
            .any(|x| *x != 0)
    }

    pub fn can_be_equal_best(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let (mut diff_bit, mut diff_sum) = (0, 0);
        target.iter().zip(arr.iter()).for_each(|(&x, &y)| {
            diff_sum += x - y;
            diff_bit ^= x ^ y;
        });
        diff_sum == 0 && diff_bit == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]));
    }

    #[test]
    fn test_2() {
        assert!(Solution::can_be_equal_optimal(vec![7], vec![7]));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::can_be_equal_best(vec![3, 7, 9], vec![3, 7, 11]));
    }
}
