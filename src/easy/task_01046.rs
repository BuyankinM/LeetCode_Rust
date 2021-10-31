// 1046. Last Stone Weight
// https://leetcode.com/problems/last-stone-weight/

use crate::Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;

        let mut h = BinaryHeap::from(stones);
        while h.len() > 1 {
            let s1 = h.pop().unwrap();
            let s2 = h.pop().unwrap();
            h.push(s1 - s2)
        }
        h.pop().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::last_stone_weight(vec![1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(79, Solution::last_stone_weight(vec![1, 100, 110, 130]));
    }
}
