// 2527. Find Xor-Beauty of Array
// https://leetcode.com/problems/find-xor-beauty-of-array/description/

use crate::Solution;

impl Solution {
    pub fn xor_beauty(nums: Vec<i32>) -> i32 {
        // nums = [a, b]
        //
        // 0: (a | a) & a = a
        // 1: (a | a) & b = a & b
        // 2: (a | b) & a = a
        // 3: (a | b) & b = b
        // 4: (b | a) & a = a
        // 5: (b | a) & b = b
        // 6: (b | b) & a = b & a
        // 7: (b | b) & b = b
        //
        // We get identical pairs with XOR = 0: 1 and 6, 2 and 4, 3 and 5
        // So, result = a ^ b (with cominations (0,0,0) and (1,1,1))
        nums.into_iter().reduce(|acc, x| acc ^ x).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::xor_beauty(vec![1, 4]));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            34,
            Solution::xor_beauty(vec![15, 45, 20, 2, 34, 35, 5, 44, 32, 30])
        );
    }
}
