// 1154. Day of the Year
// https://leetcode.com/problems/day-of-the-year/

use crate::Solution;

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let date_parts = date
            .split('-')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let (year, month, day) = (date_parts[0], date_parts[1], date_parts[2]);
        let leap_year_day = month > 2 && year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
        let days_in_months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        days_in_months[..month as usize - 1].iter().sum::<i32>() + day + leap_year_day as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(9, Solution::day_of_year("2019-01-09".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(41, Solution::day_of_year("2019-02-10".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(60, Solution::day_of_year("2003-03-01".to_string()));
    }

    #[test]
    fn test_4() {
        assert_eq!(61, Solution::day_of_year("2004-03-01".to_string()));
    }
}
