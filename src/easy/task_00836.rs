// 836. Rectangle Overlap
// https://leetcode.com/problems/rectangle-overlap/

use crate::Solution;

impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        let (x01, y01, x02, y02) = (rec1[0], rec1[1], rec1[2], rec1[3]);
        let (x11, y11, x12, y12) = (rec2[0], rec2[1], rec2[2], rec2[3]);
        !(x01 >= x12 || y01 >= y12 || x02 <= x11 || y02 <= y11)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_rectangle_overlap(
            vec![0, 0, 2, 2],
            vec![1, 1, 3, 3]
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_rectangle_overlap(
            vec![0, 0, 1, 1],
            vec![1, 0, 2, 1]
        ));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::is_rectangle_overlap(
            vec![0, 0, 1, 1],
            vec![2, 2, 3, 3]
        ));
    }
}
