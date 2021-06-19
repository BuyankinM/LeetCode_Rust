// 1619. Mean of Array After Removing Some Elements
// https://leetcode.com/problems/mean-of-array-after-removing-some-elements/

use crate::Solution;

impl Solution {
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        let (l, l_5per) = (arr.len(), arr.len() / 20);
        arr.sort_unstable();
        arr[l_5per..(l - l_5per)]
            .iter()
            .map(|x| *x as f64)
            .sum::<f64>()
            / (l - 2 * l_5per) as f64
    }

    pub fn trim_mean_no_map(mut arr: Vec<i32>) -> f64 {
        let (l, l_5per) = (arr.len(), arr.len() / 20);
        arr.sort_unstable();
        arr[l_5per..(l - l_5per)].iter().sum::<i32>() as f64 / (l - 2 * l_5per) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2.0,
            Solution::trim_mean(vec![
                1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            4.0,
            Solution::trim_mean(vec![
                6, 2, 7, 5, 1, 2, 0, 3, 10, 2, 5, 0, 5, 5, 0, 8, 7, 6, 8, 0
            ])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            5.277777777777778,
            Solution::trim_mean_no_map(vec![
                9, 7, 8, 7, 7, 8, 4, 4, 6, 8, 8, 7, 6, 8, 8, 9, 2, 6, 0, 0, 1, 10, 8, 6, 3, 3, 5,
                1, 10, 9, 0, 7, 10, 0, 10, 4, 1, 10, 6, 9, 3, 6, 0, 0, 2, 7, 0, 6, 7, 2, 9, 7, 7,
                3, 0, 1, 6, 1, 10, 3
            ])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            5.291666666666667,
            Solution::trim_mean_no_map(vec![
                4, 8, 4, 10, 0, 7, 1, 3, 7, 8, 8, 3, 4, 1, 6, 2, 1, 1, 8, 0, 9, 8, 0, 3, 9, 10, 3,
                10, 1, 10, 7, 3, 2, 1, 4, 9, 10, 7, 6, 4, 0, 8, 5, 1, 2, 1, 6, 2, 5, 0, 7, 10, 9,
                10, 3, 7, 10, 5, 8, 5, 7, 6, 7, 6, 10, 9, 5, 10, 5, 5, 7, 2, 10, 7, 7, 8, 2, 0, 1,
                1
            ])
        );
    }
}
