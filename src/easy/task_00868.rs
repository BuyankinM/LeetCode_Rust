// 868. Binary Gap
// https://leetcode.com/problems/binary-gap/

use crate::Solution;

impl Solution {
    pub fn binary_gap(mut n: i32) -> i32 {
        let (mut max_dist, mut cur_dist) = (0, 0);
        n >>= n.trailing_zeros(); // drop zeros 101000 => 101

        while n > 0 {
            if n & 1 == 1 && cur_dist > 0 {
                max_dist = max_dist.max(cur_dist);
                cur_dist = 0;
            }
            cur_dist += 1;
            n >>= 1;
        }
        max_dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::binary_gap(22));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::binary_gap(5));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::binary_gap(6));
    }

    #[test]
    fn test_4() {
        assert_eq!(0, Solution::binary_gap(8));
    }

    #[test]
    fn test_5() {
        assert_eq!(0, Solution::binary_gap(1));
    }
}
