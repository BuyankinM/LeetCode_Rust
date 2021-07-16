// 1417. Reformat The String
// https://leetcode.com/problems/reformat-the-string/

use crate::Solution;

impl Solution {
    pub fn reformat(s: String) -> String {
        let num_digits = s.chars().filter(|c| c.is_ascii_digit()).count() as i32;
        let num_chars = s.len() as i32 - num_digits;

        if (num_digits - num_chars).abs() > 1 {
            return "".to_owned();
        }

        let mut res_vec = vec![' '; s.len()];
        let (mut ind_letter, mut ind_digit) = match num_chars >= num_digits {
            true => (0, 1),
            false => (1, 0),
        };

        for c in s.chars() {
            match c.is_ascii_digit() {
                true => {
                    res_vec[ind_digit] = c;
                    ind_digit += 2
                }
                false => {
                    res_vec[ind_letter] = c;
                    ind_letter += 2
                }
            }
        }
        res_vec.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("a0b1c2".to_owned(), Solution::reformat("a0b1c2".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!("".to_owned(), Solution::reformat("leetcode".to_owned()));
    }

    #[test]
    fn test_3() {
        assert_eq!("".to_owned(), Solution::reformat("1229857369".to_owned()));
    }
}
