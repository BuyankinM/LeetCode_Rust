// 1962. Remove Stones to Minimize the Total
// https://leetcode.com/problems/remove-stones-to-minimize-the-total/description/

use crate::Solution;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut heap = std::collections::BinaryHeap::from(piles);
        (0..k).for_each(|_| {
            if let Some(pile) = heap.pop() {
                heap.push(pile - pile / 2);
            }
        });
        heap.into_iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(12, Solution::min_stone_sum(vec![5, 4, 9], 2));
    }

    #[test]
    fn test_2() {
        assert_eq!(12, Solution::min_stone_sum(vec![4, 3, 6, 7], 3));
    }
}
