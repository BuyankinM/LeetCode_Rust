// 1232. Check If It Is a Straight Line
// https://leetcode.com/problems/check-if-it-is-a-straight-line/

use crate::Solution;

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        if coordinates.len() == 2 {
            return true;
        }

        // check k1 == k2 for 3 points => (y2 - y1) / (x2 - x1) == (y1 - y0) / (x1 - x0)
        // transform to mul => (y2 - y1) * (x1 - x0) == (x2 - x1) * (y1 - y0)
        coordinates.windows(3).all(|points| {
            let (p2, p1, p0) = (&points[2], &points[1], &points[0]);
            (p2[1] - p1[1]) * (p1[0] - p0[0]) == (p2[0] - p1[0]) * (p1[1] - p0[1])
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::check_straight_line(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7]
        ]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::check_straight_line(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7]
        ]));
    }
}
