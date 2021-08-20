// 1266. Minimum Time Visiting All Points
// https://leetcode.com/problems/minimum-time-visiting-all-points/

use crate::Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 1 {
            return 0;
        };

        let mut dist = 0;
        let mut prev_point = &points[0];
        points[1..].iter().for_each(|point| {
            dist += (point[0] - prev_point[0])
                .abs()
                .max((point[1] - prev_point[1]).abs());
            prev_point = point;
        });
        dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            7,
            Solution::min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            5,
            Solution::min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]])
        );
    }
}
