// 2605. Form Smallest Number From Two Digit Arrays
// https://leetcode.com/problems/form-smallest-number-from-two-digit-arrays/

use crate::Solution;

impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::collections::BTreeSet;
        let set_1 = nums1.into_iter().collect::<BTreeSet<_>>();
        let set_2 = nums2.into_iter().collect::<BTreeSet<_>>();
        match set_1.intersection(&set_2).next() {
            Some(&n) => n,
            _ => {
                let (m1, m2) = (*set_1.first().unwrap(), *set_2.first().unwrap());
                10 * m1.min(m2) + m1.max(m2)
            }
        }
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(15, Solution::min_number(vec![4, 1, 3], vec![5, 7]));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::min_number(vec![3, 5, 2, 6], vec![3, 1, 7]));
    }
}
