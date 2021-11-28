// 806. Number of Lines To Write String
// https://leetcode.com/problems/number-of-lines-to-write-string/

use crate::Solution;

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let (mut num_lines, mut last_line) = (0, 0);
        for width in s.as_bytes().iter().map(|b| widths[(*b - b'a') as usize]) {
            last_line += width;
            if last_line > 100 {
                last_line = width;
                num_lines += 1;
            }
        }
        vec![num_lines + 1, last_line]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![3, 60],
            Solution::number_of_lines(
                vec![
                    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                "abcdefghijklmnopqrstuvwxyz".to_string()
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![2, 4],
            Solution::number_of_lines(
                vec![
                    4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                "bbbcccdddaaa".to_string()
            )
        );
    }
}
