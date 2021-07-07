// 1051. Height Checker
// https://leetcode.com/problems/height-checker/

use crate::Solution;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut heights_sort = heights.clone();
        heights_sort.sort_unstable();
        heights
            .iter()
            .zip(heights_sort.iter())
            .filter(|(h, hs)| h != hs)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::height_checker(vec![1, 1, 4, 2, 1, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(5, Solution::height_checker(vec![5, 1, 2, 3, 4]));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::height_checker(vec![1, 2, 3, 4, 5]));
    }
}
