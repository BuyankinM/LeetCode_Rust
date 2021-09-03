// 1200. Minimum Absolute Difference
// https://leetcode.com/problems/minimum-absolute-difference/

use crate::Solution;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();

        let min_diff = arr.windows(2).map(|pair| pair[1] - pair[0]).min().unwrap();

        arr.windows(2)
            .filter_map(|pair| match (pair[1] - pair[0]) == min_diff {
                true => Some(pair.to_vec()),
                false => None,
            })
            .collect::<Vec<_>>()
    }

    pub fn minimum_abs_difference_opt(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;

        arr.sort_unstable();

        let mut res = vec![];
        let mut min_diff = i32::MAX;

        for (diff, pair) in arr.windows(2).map(|pair| (pair[1] - pair[0], pair)) {
            match diff.cmp(&min_diff) {
                Ordering::Equal => res.push(pair.to_vec()),
                Ordering::Less => {
                    min_diff = diff;
                    res = vec![pair.to_vec()]
                }
                _ => (),
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![vec![1, 2], vec![2, 3], vec![3, 4]],
            Solution::minimum_abs_difference(vec![4, 2, 1, 3])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![vec![1, 2], vec![2, 3], vec![3, 4]],
            Solution::minimum_abs_difference_opt(vec![4, 2, 1, 3])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![vec![-14, -10], vec![19, 23], vec![23, 27]],
            Solution::minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            vec![vec![-14, -10], vec![19, 23], vec![23, 27]],
            Solution::minimum_abs_difference_opt(vec![3, 8, -10, 23, 19, -4, -14, 27])
        );
    }
}
