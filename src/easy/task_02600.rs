// 2600. K Items With the Maximum Sum
// https://leetcode.com/problems/k-items-with-the-maximum-sum/

use crate::Solution;

impl Solution {
    pub fn k_items_with_maximum_sum(
        num_ones: i32,
        num_zeros: i32,
        num_neg_ones: i32,
        mut k: i32,
    ) -> i32 {
        let ones = k.min(num_ones);
        k -= ones;
        k -= k.min(num_zeros);
        ones - k.min(num_neg_ones)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::k_items_with_maximum_sum(3, 2, 0, 2));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::k_items_with_maximum_sum(3, 2, 0, 4));
    }
}
