// 1394. Find Lucky Integer in an Array
// https://leetcode.com/problems/find-lucky-integer-in-an-array/

use crate::Solution;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut counter = [0_usize; 501];
        arr.iter().for_each(|&x| counter[x as usize] += 1);

        counter
            .iter()
            .enumerate()
            .skip(1)
            .rev()
            .find_map(|(ind, x)| match ind == *x {
                true => Some(*x as i32),
                false => None,
            })
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::find_lucky(vec![2, 2, 3, 4]));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]));
    }

    #[test]
    fn test_3() {
        assert_eq!(-1, Solution::find_lucky(vec![2, 2, 2, 3, 3]));
    }
}
