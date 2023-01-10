// 2530. Maximal Score After Applying K Operations
// https://leetcode.com/problems/maximal-score-after-applying-k-operations/

use crate::Solution;

const DIVISOR: i32 = 3;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut heap = std::collections::BinaryHeap::from(nums);
        (0..k).fold(0, |res, _| {
            let value = heap.pop().unwrap();
            heap.push((value + DIVISOR - 1) / DIVISOR);
            res + value as i64
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(50, Solution::max_kelements(vec![10, 10, 10, 10, 10], 5));
    }

    #[test]
    fn test_2() {
        assert_eq!(17, Solution::max_kelements(vec![1, 10, 3, 3, 3], 3));
    }
}
