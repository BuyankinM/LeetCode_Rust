// 1346. Check If N and Its Double Exist
// https://leetcode.com/problems/check-if-n-and-its-double-exist/

use crate::Solution;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let mut doubles = HashSet::new();

        for val in arr {
            if doubles.contains(&val) {
                return true;
            }
            if val % 2 == 0 {
                doubles.insert(val / 2);
            }
            doubles.insert(val * 2);
        }

        false
    }

    pub fn check_if_exist_any(arr: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::new();
        arr.iter().any(|&x| {
            let found = set.contains(&(x * 2)) || x % 2 == 0 && set.contains(&(x / 2));
            set.insert(x);
            found
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::check_if_exist(vec![10, 2, 5, 3]));
    }

    #[test]
    fn test_2() {
        assert!(Solution::check_if_exist(vec![7, 1, 14, 11]));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::check_if_exist(vec![3, 1, 7, 11]));
    }
}
