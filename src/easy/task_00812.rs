// 812. Largest Triangle Area
// https://leetcode.com/problems/largest-triangle-area/

use crate::Solution;

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut s = f64::MIN;
        for (i, p1) in points.iter().enumerate() {
            for (j, p2) in points[i + 1..].iter().enumerate() {
                for p3 in points[j + 1..].iter() {
                    if let [x1, y1] = p1[..] {
                        if let [x2, y2] = p2[..] {
                            if let [x3, y3] = p3[..] {
                                s = s.max(
                                    0.5 * ((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)).abs()
                                        as f64,
                                );
                            }
                        }
                    }
                }
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ans = 2.0;
        let res = Solution::largest_triangle_area(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![0, 2],
            vec![2, 0],
        ]);
        assert!((ans - res).abs() < f64::EPSILON);
    }

    #[test]
    fn test_2() {
        let ans = 0.5;
        let res = Solution::largest_triangle_area(vec![vec![1, 0], vec![0, 0], vec![0, 1]]);
        assert!((ans - res).abs() < f64::EPSILON);
    }
}
