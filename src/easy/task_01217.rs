// 1217. Minimum Cost to Move Chips to The Same Position
// https://leetcode.com/problems/minimum-cost-to-move-chips-to-the-same-position/

use crate::Solution;

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        *position
            .iter()
            .fold([0, 0], |mut acc, &pos| {
                acc[(pos % 2) as usize] += 1;
                acc
            })
            .iter()
            .min()
            .unwrap() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::min_cost_to_move_chips(vec![1, 2, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::min_cost_to_move_chips(vec![1, 2, 3, 3]));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::min_cost_to_move_chips(vec![1, 3, 5]));
    }
}
