// 2525. Categorize Box According to Criteria
// https://leetcode.com/problems/categorize-box-according-to-criteria/

use crate::Solution;

const MAX_DIM: i32 = 10_000;
const MAX_VOL: i64 = 1_000_000_000;
const MAX_MASS: i32 = 100;

impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        let is_bulky = length >= MAX_DIM
            || width >= MAX_DIM
            || height >= MAX_DIM
            || (length as i64) * (width as i64) * (height as i64) >= MAX_VOL;

        let is_heavy = mass >= MAX_MASS;
        match (is_bulky, is_heavy) {
            (true, true) => "Both".to_string(),
            (true, false) => "Bulky".to_string(),
            (false, true) => "Heavy".to_string(),
            _ => "Neither".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "Heavy".to_string(),
            Solution::categorize_box(1000, 35, 700, 300)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "Neither".to_string(),
            Solution::categorize_box(200, 50, 800, 50)
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            "Both".to_string(),
            Solution::categorize_box(2909, 3968, 3272, 727)
        );
    }
}
