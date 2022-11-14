// 2469. Convert the Temperature
// https://leetcode.com/problems/convert-the-temperature/

use crate::Solution;

impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        vec![celsius + 273.15, celsius * 1.80 + 32.00]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::convert_temperature(36.50), vec![309.65, 97.7]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::convert_temperature(122.11), vec![395.26, 251.798]);
    }
}
