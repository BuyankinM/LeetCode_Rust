// 1385. Find the Distance Value Between Two Arrays
// https://leetcode.com/problems/find-the-distance-value-between-two-arrays/

use crate::Solution;

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, mut arr2: Vec<i32>, d: i32) -> i32 {
        arr2.sort_unstable();
        arr1.iter().fold(0, |res, &x| {
            res + (arr2.iter().all(|&y| (x - y).abs() > d) as i32)
        })
    }

    pub fn find_the_distance_value_optimal(mut arr1: Vec<i32>, mut arr2: Vec<i32>, d: i32) -> i32 {
        arr1.sort_unstable();
        arr2.sort_unstable();

        let (mut res, mut i, mut j) = (0, 0, 0);

        while i < arr1.len() && j < arr2.len() {
            if arr1[i] <= arr2[j] + d {
                if arr1[i] < arr2[j] - d {
                    res += 1;
                }
                i += 1;
            } else {
                j += 1;
            }
        }

        (res + arr1.len() - i) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            2,
            Solution::find_the_distance_value(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3)
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            1,
            Solution::find_the_distance_value(vec![2, 1, 100, 3], vec![-5, -2, 10, -3, 7], 6)
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            1,
            Solution::find_the_distance_value_optimal(
                vec![2, 1, 100, 3],
                vec![-5, -2, 10, -3, 7],
                6
            )
        );
    }
}
