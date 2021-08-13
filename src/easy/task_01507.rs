// 1507. Reformat Date
// https://leetcode.com/problems/reformat-date/

use crate::Solution;

impl Solution {
    pub fn reformat_date(date: String) -> String {
        let map_months = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ]
        .iter()
        .enumerate()
        .map(|(ind, s)| (*s, ind + 1))
        .collect::<std::collections::HashMap<&str, usize>>();

        date.rsplit(' ')
            .enumerate()
            .map(|(ind, s)| match ind {
                2 => format!("{:0>2}", s.replace(char::is_alphabetic, "")),
                1 => format!("{:02}", map_months[s]),
                _ => s.to_owned(),
            })
            .collect::<Vec<_>>()
            .join("-")
    }

    pub fn reformat_date_list(date: String) -> String {
        let arr_months = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];
        date.rsplit(' ')
            .enumerate()
            .map(|(ind, s)| match ind {
                2 => format!("{:0>2}", s.replace(char::is_alphabetic, "")),
                1 => format!(
                    "{:02}",
                    arr_months.iter().position(|x| *x == s).unwrap() + 1
                ),
                _ => s.to_owned(),
            })
            .collect::<Vec<_>>()
            .join("-")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "2052-10-20".to_owned(),
            Solution::reformat_date("20th Oct 2052".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "1933-06-06".to_owned(),
            Solution::reformat_date("6th Jun 1933".to_owned())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "1960-05-26".to_owned(),
            Solution::reformat_date_list("26th May 1960".to_owned())
        );
    }
}
