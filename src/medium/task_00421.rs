// 421. Maximum XOR of Two Numbers in an Array
// https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/

use crate::Solution;

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 2],
}

impl Solution {
    // https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/discuss/849128/Python-O(32n)-solution-explained
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let (mut res, mut mask) = (0, 0);
        for i in (0..32).rev() {
            mask |= 1 << i;
            let found: HashSet<i32> = nums.iter().map(|&x| x & mask).collect();
            let start = res | 1 << i;
            for &pref in found.iter() {
                if found.contains(&(start ^ pref)) {
                    res = start;
                    break;
                }
            }
        }
        res
    }

    // https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/discuss/849356/Rust-Trie-solution
    pub fn find_maximum_xor_trie(nums: Vec<i32>) -> i32 {
        let mut trie: Trie = Default::default();
        for &num in nums.iter() {
            let mut node = &mut trie;
            for i in (0..31).rev() {
                let bit = ((num >> i) & 1) as usize;
                node = node.children[bit].get_or_insert_with(Default::default);
            }
        }
        let mut answer = 0;
        for &num in nums.iter() {
            let mut max = 0;
            let mut node = &trie;
            for i in (0..31).rev() {
                let bit = ((num >> i) & 1) as usize;
                if let Some(n) = &node.children[1 - bit] {
                    max |= 1 << i;
                    node = n;
                } else {
                    node = node.children[bit].as_ref().unwrap();
                }
            }
            answer = std::cmp::max(answer, max);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(28, Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            127,
            Solution::find_maximum_xor(vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(0b111, Solution::find_maximum_xor(vec![0b101, 0b010, 0b001]));
    }
}
