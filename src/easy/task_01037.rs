// 1037. Valid Boomerang
// https://leetcode.com/problems/valid-boomerang/

use crate::Solution;

impl Solution {
    pub fn is_boomerang(p: Vec<Vec<i32>>) -> bool {
        let (x0, y0) = (p[0][0], p[0][1]);
        let (x1, y1) = (p[1][0], p[1][1]);
        let (x2, y2) = (p[2][0], p[2][1]);

        ((x0 - x1) * (y2 - y1) - (x2 - x1) * (y0 - y1)).abs() != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_boomerang(vec![
            vec![1, 1],
            vec![2, 3],
            vec![3, 2]
        ]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_boomerang(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 3]
        ]));
    }
}
