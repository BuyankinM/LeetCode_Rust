// 645. Set Mismatch
// https://leetcode.com/problems/set-mismatch/

use crate::Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0, 0];
        let mut counter = [0; 10_000];
        nums.iter().for_each(|&x| counter[(x - 1) as usize] += 1);
        counter[..nums.len()]
            .iter()
            .zip(1..)
            .for_each(|(x, i)| match *x {
                0 => res[1] = i,
                2 => res[0] = i,
                _ => (),
            });
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![2, 3], Solution::find_error_nums(vec![1, 2, 2, 4]));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![1, 2], Solution::find_error_nums(vec![1, 1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![2, 1], Solution::find_error_nums(vec![2, 2]));
    }
}
