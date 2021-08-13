// 66. Plus One
// https://leetcode.com/problems/plus-one/

use crate::Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for x in digits.iter_mut().rev() {
            match *x == 9 {
                true => *x = 0,
                false => {
                    *x += 1;
                    return digits;
                }
            }
        }
        digits.insert(0, 1);
        digits
    }

    pub fn plus_one_tryfold(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;

        if digits
            .iter_mut()
            .rev()
            .try_fold(1, |carry, v| {
                *v += carry;
                if *v == 10 {
                    *v = 0;
                    Some(1)
                } else {
                    None // stop since no more carry
                }
            })
            .is_some()
        {
            digits.insert(0, 1);
        }
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![1, 2, 4], Solution::plus_one(vec![1, 2, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![4, 3, 2, 2], Solution::plus_one(vec![4, 3, 2, 1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![1], Solution::plus_one(vec![0]));
    }

    #[test]
    fn test_4() {
        assert_eq!(vec![1, 0, 0], Solution::plus_one(vec![9, 9]));
    }
}
