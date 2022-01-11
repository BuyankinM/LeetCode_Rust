// 2133. Check if Every Row and Column Contains All Numbers
// https://leetcode.com/problems/check-if-every-row-and-column-contains-all-numbers/

use crate::Solution;

impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let l = matrix.len();
        for dir in 0..2 {
            let mut counter = vec![0_usize; l + 1];
            for i in 0..l {
                for j in 0..l {
                    let val = match dir {
                        0 => matrix[i][j], // check rows
                        _ => matrix[j][i], // check columns
                    };
                    let cur_num = &mut counter[val as usize];
                    match *cur_num == i {
                        true => *cur_num += 1,
                        false => return false,
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::check_valid(vec![
            vec![1, 2, 3],
            vec![3, 1, 2],
            vec![2, 3, 1]
        ]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::check_valid(vec![
            vec![1, 1, 1],
            vec![1, 2, 3],
            vec![1, 2, 3]
        ]));
    }
}
