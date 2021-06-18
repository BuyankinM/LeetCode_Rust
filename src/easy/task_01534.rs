// 1534. Count Good Triplets
// https://leetcode.com/problems/count-good-triplets/

use crate::Solution;

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut counter = 0;
        for i in 0..arr.len() - 2 {
            for j in i + 1..arr.len() - 1 {
                if (arr[i] - arr[j]).abs() > a {
                    continue;
                }
                for k in j + 1..arr.len() {
                    if (arr[j] - arr[k]).abs() > b || (arr[i] - arr[k]).abs() > c {
                        continue;
                    }
                    counter += 1;
                }
            }
        }
        counter
    }

    pub fn count_good_triplets_1(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut counter = 0;

        for i in 0..arr.len() - 2 {
            for j in i + 1..arr.len() - 1 {
                if (arr[i] - arr[j]).abs() > a {
                    continue;
                }
                for k in j + 1..arr.len() {
                    if (arr[j] - arr[k]).abs() <= b && (arr[i] - arr[k]).abs() <= c {
                        counter += 1;
                    }
                }
            }
        }
        counter
    }

    pub fn count_good_triplets_func(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut counter = 0;
        let l = arr.len();

        arr[..l - 2].iter().enumerate().for_each(|(i, &ai)| {
            arr[..l - 1]
                .iter()
                .enumerate()
                .skip(i + 1)
                .filter(|(_, aj)| (ai - **aj).abs() <= a)
                .for_each(|(j, &aj)| {
                    arr[j + 1..l]
                        .iter()
                        .filter(|&&ak| (aj - ak).abs() <= b && (ai - ak).abs() <= c)
                        .for_each(|_| {
                            counter += 1;
                        })
                })
        });
        counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            4,
            Solution::count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            0,
            Solution::count_good_triplets_1(vec![1, 1, 2, 2, 3], 0, 0, 1)
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            5,
            Solution::count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 5)
        );
    }
}
