// 1539. Kth Missing Positive Number
// https://leetcode.com/problems/kth-missing-positive-number/

use crate::Solution;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, mut k: i32) -> i32 {
        let (mut i, mut val) = (0, 1);
        loop {
            match i < arr.len() {
                true if val == arr[i] => i += 1,
                _ => {
                    k -= 1;
                    if k == 0 {
                        break val;
                    }
                }
            }
            val += 1;
        }
    }

    pub fn find_kth_positive_binary_search(arr: Vec<i32>, k: i32) -> i32 {
        let (mut l, mut r) = (0, arr.len());
        while l < r {
            let m = l + (r - l) / 2;
            if arr[m] - m as i32 - 1 >= k {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l as i32 + k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(9, Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 5));
    }

    #[test]
    fn test_2() {
        assert_eq!(6, Solution::find_kth_positive(vec![1, 2, 3, 4], 2));
    }
}
