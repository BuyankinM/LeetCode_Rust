// 1790. Check if One String Swap Can Make Strings Equal
// https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/

use crate::Solution;

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut n = 0;
        let mut c_swap = (' ', ' ');

        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 != c2 {
                n += 1;

                if n == 1 {
                    c_swap = (c1, c2);
                } else if n == 2 && c_swap != (c2, c1) || n > 2 {
                    return false;
                }
            }
        }

        n != 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            true,
            Solution::are_almost_equal("bank".to_owned(), "kanb".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            false,
            Solution::are_almost_equal("attack".to_owned(), "defend".to_owned())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            true,
            Solution::are_almost_equal("kelb".to_owned(), "kelb".to_owned())
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            false,
            Solution::are_almost_equal("abcd".to_owned(), "dcba".to_owned())
        );
    }
}
