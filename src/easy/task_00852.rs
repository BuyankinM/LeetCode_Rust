// 852. Peak Index in a Mountain Array
// https://leetcode.com/problems/peak-index-in-a-mountain-array/

use crate::Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        arr.iter()
            .zip(arr[1..].iter())
            .position(|(a, b)| a > b)
            .unwrap() as _
    }

    // https://leetcode.com/problems/peak-index-in-a-mountain-array/discuss/693973/Rust-or-Scan-Max-or-Binary-Search-or-0ms
    pub fn peak_index_in_mountain_array_bs(a: Vec<i32>) -> i32 {
        let (mut low, mut high) = (0, a.len() - 1);
        let mut med;
        loop {
            med = (low + high) / 2;
            match (a[med - 1], a[med], a[med + 1]) {
                (prev, cur, next) if prev > cur && cur > next => high = med,
                (prev, cur, next) if prev < cur && cur < next => low = med,
                _ => break,
            }
        }
        med as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::peak_index_in_mountain_array(vec![0, 1, 0]));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            1,
            Solution::peak_index_in_mountain_array_bs(vec![0, 10, 5, 2])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            2,
            Solution::peak_index_in_mountain_array_bs(vec![3, 4, 5, 1])
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            2,
            Solution::peak_index_in_mountain_array_bs(vec![
                24, 69, 100, 99, 79, 78, 67, 36, 26, 19
            ])
        );
    }
}
