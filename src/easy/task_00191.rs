// 191. Number of 1 Bits
// https://leetcode.com/problems/number-of-1-bits/

use crate::Solution;

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let mut count = 0;
        let mut n = n;
        while n > 0 {
            count += n & 1;
            n >>= 1;
        }
        count as _
    }

    pub fn hammingWeight_oneliner(n: u32) -> i32 {
        n.count_ones() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::hammingWeight(0b1011), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::hammingWeight(0b10000000), 1);
    }
}
