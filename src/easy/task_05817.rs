// 5817. Check If a String is Decomposble to Value-Equal Substrings
// https://leetcode.com/problems/check-if-a-string-is-decomposble-to-value-equal-substrings/

use crate::Solution;

impl Solution {
    pub fn is_decomposable(s: String) -> bool {
        let mut size_two = false;
        let mut counter = 0;
        let mut prev_c = None;

        for opt_c in s.chars().map(|x| Some(x)) {
            if prev_c.is_some() && prev_c != opt_c {
                match counter % 3 {
                    rem if rem == 1 || rem == 2 && size_two => return false,
                    2 => size_two = true,
                    _ => (),
                };
                counter = 0;
            }
            prev_c = opt_c;
            counter += 1;
        }
        size_two || counter % 3 == 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(false, Solution::is_decomposable("000111000".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(true, Solution::is_decomposable("00011111222".to_owned()));
    }
    #[test]
    fn test_3() {
        assert_eq!(
            false,
            Solution::is_decomposable("01110002223300".to_owned())
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            false,
            Solution::is_decomposable("01110002223300".to_owned())
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            true,
            Solution::is_decomposable("88888866666611177333".to_owned())
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            true,
            Solution::is_decomposable("22222222222222222222222".to_owned())
        );
    }

    #[test]
    fn test_7() {
        assert_eq!(true, Solution::is_decomposable("00".to_owned()));
    }
}
