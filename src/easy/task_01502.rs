// 1502. Can Make Arithmetic Progression From Sequence
// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/

use crate::Solution;

impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();

        let delta = arr[1] - arr[0];

        arr.iter()
            .enumerate()
            .skip(1)
            .all(|(ind, val)| val - arr[ind - 1] == delta)
    }

    pub fn can_make_arithmetic_progression_hashset(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();
        (1..arr.len())
            .map(|i| arr[i] - arr[i - 1])
            .collect::<std::collections::HashSet<i32>>()
            .len()
            == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::can_make_arithmetic_progression(vec![3, 5, 1]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::can_make_arithmetic_progression_hashset(vec![
            1, 2, 4
        ]));
    }
}
