// 1089. Duplicate Zeros
// https://leetcode.com/problems/duplicate-zeros/

use crate::Solution;

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut i: usize = 0;
        while i < (arr.len() - 1) {
            let val = arr[i];
            if val == 0 {
                arr.insert(i + 1, 0);
                arr.pop();
                i += 1;
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut v = vec![1, 0, 2, 3, 0, 4, 5, 0];
        Solution::duplicate_zeros(&mut v);
        assert_eq!(vec![1, 0, 0, 2, 3, 0, 0, 4], v);
    }

    #[test]
    fn test_2() {
        let mut v = vec![1, 2, 3];
        Solution::duplicate_zeros(&mut v);
        assert_eq!(vec![1, 2, 3], v);
    }
}
