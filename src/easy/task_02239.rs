// 2239. Find Closest Number to Zero
// https://leetcode.com/problems/find-closest-number-to-zero/

use crate::Solution;

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        use std::cmp::Ordering;
        nums.iter()
            .fold((i32::MIN, i32::MAX), |(val, dist), &x| {
                match dist.cmp(&(x.abs())) {
                    Ordering::Greater => (x, x.abs()),
                    Ordering::Equal => (x.max(val), dist),
                    Ordering::Less => (val, dist),
                }
            })
            .0
    }

    pub fn find_closest_number_short(nums: Vec<i32>) -> i32 {
        use std::cmp::Ordering;
        nums.into_iter()
            .max_by(|x, y| match y.abs().cmp(&(x.abs())) {
                Ordering::Equal => x.cmp(y),
                abs_cmp => abs_cmp,
            })
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_closest_number(vec![-4, -2, 1, 4, 8]), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_closest_number(vec![2, -1, 1]), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::find_closest_number(vec![-1, 2, 1, -4]), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::find_closest_number(vec![-1, 2, 0, 1, -4]), 0);
    }
}
