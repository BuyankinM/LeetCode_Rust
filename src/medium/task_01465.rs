// 1465. Maximum Area of a Piece of Cake After Horizontal and Vertical Cuts
// https://leetcode.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/

use crate::Solution;

impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        let (mut hv, mut vv) = (horizontal_cuts, vertical_cuts);
        hv.extend([0, h]);
        vv.extend([0, w]);
        hv.sort_unstable();
        vv.sort_unstable();

        let max_delta_h = hv.windows(2).fold(0, |delta, v| delta.max(v[1] - v[0])) as u64;
        let max_delta_w = vv.windows(2).fold(0, |delta, v| delta.max(v[1] - v[0])) as u64;

        (max_delta_h * max_delta_w % 1_000_000_007) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::max_area(5, 4, vec![1, 2, 4], vec![1, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(6, Solution::max_area(5, 4, vec![3, 1], vec![1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(9, Solution::max_area(5, 4, vec![3], vec![3]));
    }

    #[test]
    fn test_4() {
        assert_eq!(
            81,
            Solution::max_area(1000000000, 1000000000, vec![2], vec![2])
        );
    }
}
