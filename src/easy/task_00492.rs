// 492. Construct the Rectangle
// https://leetcode.com/problems/construct-the-rectangle/

use crate::Solution;

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut x = (area as f32).sqrt() as i32;
        loop {
            match area % x {
                0 => break vec![area / x, x],
                _ => x -= 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![2, 2], Solution::construct_rectangle(4));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![37, 1], Solution::construct_rectangle(37));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![427, 286], Solution::construct_rectangle(122122));
    }
}
