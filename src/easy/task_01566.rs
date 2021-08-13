// 1566. Detect Pattern of Length M Repeated K or More Times
// https://leetcode.com/problems/detect-pattern-of-length-m-repeated-k-or-more-times/

use crate::Solution;

impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let (mut res, mut prev, mut count) = (false, None, 0);

        if arr.len() < (m * k) as usize {
            return res;
        }

        'main: for i in 0..=(arr.len() - (m * k) as usize) {
            for subset in arr[i..].chunks_exact(m as usize) {
                let cur = Some(subset);
                match prev == cur {
                    true => count += 1,
                    false => count = 0,
                }

                if count + 1 == k {
                    res = true;
                    break 'main;
                }
                prev = cur;
            }
            prev = None;
        }
        res
    }

    pub fn contains_pattern_optimal(arr: Vec<i32>, m: i32, k: i32) -> bool {
        if arr.len() < (m * k) as usize {
            return false;
        }
        let (mut res, mut count) = (false, 0);

        for i in 0..(arr.len() - m as usize) {
            match arr[i] == arr[i + m as usize] {
                true => count += 1,
                false => count = 0,
            }

            if count == m * (k - 1) {
                res = true;
                break;
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
        assert!(Solution::contains_pattern(vec![1, 2, 4, 4, 4, 4], 1, 3));
    }

    #[test]
    fn test_2() {
        assert!(Solution::contains_pattern(
            vec![1, 2, 1, 2, 1, 1, 1, 3],
            2,
            2
        ));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::contains_pattern_optimal(
            vec![1, 2, 1, 2, 1, 3],
            2,
            3
        ));
    }

    #[test]
    fn test_4() {
        assert!(!Solution::contains_pattern_optimal(
            vec![1, 2, 3, 1, 2],
            2,
            2
        ));
    }
}
