// 2269. Find the K-Beauty of a Number
// https://leetcode.com/problems/find-the-k-beauty-of-a-number/

use crate::Solution;

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let mut res = 0;
        let mut n = num;
        let div = 10_i32.pow(k as u32);
        let min_val = div / 10;
        while n >= min_val {
            let x = n % div;
            if x > 0 && num % x == 0 {
                res += 1;
            }
            n /= 10;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::divisor_substrings(240, 2), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::divisor_substrings(430043, 2), 2);
    }
}
