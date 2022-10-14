// 2433. Find The Original Array of Prefix Xor
// https://leetcode.com/problems/find-the-original-array-of-prefix-xor/

use crate::Solution;

impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        pref.iter()
            .scan(0, |prev, &x| {
                let res = *prev ^ x;
                *prev = x;
                Some(res)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_array(vec![5, 2, 0, 3, 1]),
            vec![5, 7, 2, 3, 2]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_array(vec![13]), vec![13]);
    }
}
