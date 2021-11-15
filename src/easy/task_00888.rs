// 888. Fair Candy Swap
// https://leetcode.com/problems/fair-candy-swap/

use crate::Solution;

impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let alice_total: i32 = alice_sizes.iter().sum();
        let bob_total: i32 = bob_sizes.iter().sum();

        let delta_box = (alice_total - bob_total) / 2;
        let bob_set = bob_sizes.iter().collect::<HashSet<_>>();

        for (alice_box, bob_box) in alice_sizes.iter().map(|&x| (x, x - delta_box)) {
            if bob_set.contains(&bob_box) {
                return vec![alice_box, bob_box];
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![1, 2],
            Solution::fair_candy_swap(vec![1, 1], vec![2, 2])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![1, 2],
            Solution::fair_candy_swap(vec![1, 2], vec![2, 3])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![2, 3], Solution::fair_candy_swap(vec![2], vec![1, 3]));
    }

    #[test]
    fn test_4() {
        assert_eq!(
            vec![5, 4],
            Solution::fair_candy_swap(vec![1, 2, 5], vec![2, 4])
        );
    }
}
