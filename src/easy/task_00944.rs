// 944. Delete Columns to Make Sorted
// https://leetcode.com/problems/delete-columns-to-make-sorted/

use crate::Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut bad_cols = vec![false; strs[0].len()];
        let mut vs = strs[0].chars().collect::<Vec<_>>();
        for s in &strs[1..] {
            vs.iter_mut()
                .zip(s.chars())
                .enumerate()
                .for_each(|(i, (c1, c2))| {
                    if !bad_cols[i] {
                        bad_cols[i] = *c1 > c2;
                        *c1 = c2;
                    }
                });
        }
        bad_cols.iter().filter(|&&x| x).count() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            1,
            Solution::min_deletion_size(vec![
                "cba".to_string(),
                "daf".to_string(),
                "ghi".to_string()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            0,
            Solution::min_deletion_size(vec!["a".to_string(), "b".to_string()])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            0,
            Solution::min_deletion_size(vec!["a".to_string(), "b".to_string()])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            3,
            Solution::min_deletion_size(vec![
                "zyx".to_string(),
                "wvu".to_string(),
                "tsr".to_string()
            ])
        );
    }
}
