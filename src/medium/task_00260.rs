// 260. Single Number III
// https://leetcode.com/problems/single-number-iii/

use crate::Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut map = std::collections::HashMap::with_capacity(nums.len() / 2);
        nums.iter().for_each(|&x| *map.entry(x).or_insert(0) += 1);
        map.iter()
            .filter_map(|(key, val)| if *val == 1 { Some(*key) } else { None })
            .collect()
    }

    // https://leetcode.com/problems/single-number-iii/discuss/1562067/Rust-XOR-and-other-bitwise-tricks-applied-Explained
    pub fn single_number_xor(nums: Vec<i32>) -> Vec<i32> {
        let xor_all = nums.iter().fold(0, |acm, &n| acm ^ n);
        let lo_bit = xor_all & -xor_all;
        let (mut n1, mut n2) = (0, 0);

        for n in nums {
            match n & lo_bit {
                0 => n1 ^= n,
                _ => n2 ^= n,
            }
        }
        vec![n1, n2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let res = Solution::single_number(vec![1, 2, 1, 3, 2, 5]);
        assert!(res == vec![3, 5] || res == vec![5, 3]);
    }

    #[test]
    fn test_2() {
        let res = Solution::single_number(vec![-1, 0]);
        assert!(res == vec![-1, 0] || res == vec![0, -1]);
    }
}
