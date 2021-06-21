// 1694. Reformat Phone Number
// https://leetcode.com/problems/reformat-phone-number/

use crate::Solution;

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let pack_num = number.replace(&[' ', '-'][..], "");
        let l = pack_num.len();
        let last_4_dig = (l % 3) == 1;

        let mut res = String::new();
        for (ind, ch) in pack_num.chars().enumerate() {
            res.push(ch);
            if (ind + 1) % 3 == 0 && ind < (l - 1) && !(last_4_dig && ind >= (l - 3))
                || last_4_dig && ind == (l - 3)
            {
                res.push('-');
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "123-456".to_owned(),
            Solution::reformat_number("1-23-45 6".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "123-45-67".to_owned(),
            Solution::reformat_number("123 4-567".to_owned())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "123-456-78".to_owned(),
            Solution::reformat_number("123 4-5678".to_owned())
        );
    }

    #[test]
    fn test_4() {
        assert_eq!("12".to_owned(), Solution::reformat_number("12".to_owned()));
    }

    #[test]
    fn test_5() {
        assert_eq!(
            "175-229-353-94-75".to_owned(),
            Solution::reformat_number("--17-5 229 35-39475 ".to_owned())
        );
    }
}
