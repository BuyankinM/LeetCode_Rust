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

    // https://leetcode.com/problems/remove-stones-to-minimize-the-total/solutions/2960645/rust-elixir-2-approaches/?orderBy=most_relevant
    pub fn min_stone_sum_counter(piles: Vec<i32>, mut k: i32) -> i32 {
        let max = *piles.iter().max().unwrap() as usize;
        let mut count = vec![0; max + 1];
        for &x in piles.iter() {
            count[x as usize] += 1;
        }
        for i in (1..=max).rev() {
            if k == 0 {
                break;
            }
            let smaller = count[i].min(k);
            count[i] -= smaller;
            count[(i + 1) / 2] += smaller;
            k -= smaller;
        }
        count.iter().enumerate().map(|(i, c)| i as i32 * c).sum()
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
