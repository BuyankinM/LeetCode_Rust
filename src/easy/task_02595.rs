// 2595. Number of Even and Odd Bits
// https://leetcode.com/problems/number-of-even-and-odd-bits/

use crate::Solution;

impl Solution {
    pub fn even_odd_bit(mut n: i32) -> Vec<i32> {
        let mut res = vec![0, 0];
        let mut pos = 0;
        while n > 0 {
            if n & 1 == 1 {
                res[pos % 2] += 1;
            }
            n >>= 1;
            pos += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![2, 0], Solution::even_odd_bit(17));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![0, 1], Solution::even_odd_bit(2));
    }
}
