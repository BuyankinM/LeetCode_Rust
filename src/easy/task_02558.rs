// 2558. Take Gifts From the Richest Pile
// https://leetcode.com/problems/take-gifts-from-the-richest-pile/

use crate::Solution;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut heap = std::collections::BinaryHeap::from(gifts);
        for _ in 0..k {
            let val = heap.pop().unwrap();
            heap.push((val as f64).sqrt() as i32);
        }
        heap.into_iter().map(|x| x as i64).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(29, Solution::pick_gifts(vec![25, 64, 9, 4, 100], 4));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, Solution::pick_gifts(vec![1, 1, 1, 1], 4));
    }
}
