// 1779. Find Nearest Point That Has the Same X or Y Coordinate
// https://leetcode.com/problems/find-nearest-point-that-has-the-same-x-or-y-coordinate/

use crate::Solution;

impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut dist_min = std::i32::MAX;
        let mut res_ind = -1;

        for (ind, pi) in points.iter().enumerate() {
            if x == pi[0] || y == pi[1] {
                let dist_cur = (x - pi[0]).abs() + (y - pi[1]).abs();
                if dist_cur < dist_min || res_ind == -1 && dist_cur == dist_min {
                    res_ind = ind as i32;
                    dist_min = dist_cur;
                    if dist_min == 0 {
                        break;
                    }
                }
            }
        }
        res_ind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::nearest_valid_point(
                3,
                4,
                vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]]
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::nearest_valid_point(3, 4, vec![vec![3, 4]]));
    }

    #[test]
    fn test_3() {
        assert_eq!(-1, Solution::nearest_valid_point(3, 4, vec![vec![2, 3]]));
    }
}
