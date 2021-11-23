// 830. Positions of Large Groups
// https://leetcode.com/problems/positions-of-large-groups/

use crate::Solution;

impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        if s.len() < 3 {
            return vec![];
        }

        let check_and_add = |i: usize, j: usize, v: &mut Vec<Vec<i32>>| {
            if i - j >= 3 {
                v.push(vec![j as i32, (i - 1) as i32])
            }
        };

        let mut res = Vec::new();
        let (mut prev_c, mut prev_i) = (' ', 0);

        for (i, c) in s.char_indices() {
            if prev_c != c {
                check_and_add(i, prev_i, &mut res);
                prev_c = c;
                prev_i = i;
            }
        }
        check_and_add(s.len(), prev_i, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![vec![3, 6]],
            Solution::large_group_positions("abbxxxxzzy".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![vec![0; 0]; 0],
            Solution::large_group_positions("abc".to_string())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![vec![0, 2]],
            Solution::large_group_positions("aaa".to_string())
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            vec![vec![3, 5], vec![6, 9], vec![12, 14]],
            Solution::large_group_positions("abcdddeeeeaabbbcd".to_string())
        );
    }
}
