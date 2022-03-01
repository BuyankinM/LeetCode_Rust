// 338. Counting Bits
// https://leetcode.com/problems/counting-bits/

use crate::Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = vec![0; n as usize + 1];
        (1..=n as usize).for_each(|i| res[i] = res[i >> 1] + (i & 1) as i32);
        res
    }

    // https://leetcode.com/problems/counting-bits/discuss/79527/Four-lines-C%2B%2B-time-O(n)-space-O(n)
    pub fn count_bits_lsb(n: i32) -> Vec<i32> {
        let mut res = vec![0; n as usize + 1];
        (1..=n as usize).for_each(|i| res[i] = res[i & (i - 1)] + 1);
        res
    }

    pub fn count_bits_func(n: i32) -> Vec<i32> {
        (0..n).map(|i| i.count_ones() as i32).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1, 1], Solution::count_bits(2));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![0, 1, 1, 2, 1, 2], Solution::count_bits(5));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2, 3, 2],
            Solution::count_bits(20)
        );
    }
}
