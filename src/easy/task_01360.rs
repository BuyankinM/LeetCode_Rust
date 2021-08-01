// 1360. Number of Days Between Two Dates
// https://leetcode.com/problems/number-of-days-between-two-dates/

use crate::Solution;

impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        if date1 == date2 {
            return 0;
        }

        let months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let is_leap_year = |x: i32| -> bool { x % 4 == 0 && (x % 100 != 0 || x % 400 == 0) };
        let month_day_calc = |year: i32, month: i32, day: i32| -> i32 {
            (0..month - 1)
                .map(|x| months[x as usize] + (x == 1 && is_leap_year(year)) as i32)
                .sum::<i32>()
                + day
        };

        let str_to_int = |s: &str| -> i32 { s.parse().unwrap() };
        let mut min_date_vec = date1.split("-").map(str_to_int).collect::<Vec<_>>();
        let mut max_date_vec = date2.split("-").map(str_to_int).collect::<Vec<_>>();
        if date1 > date2 {
            std::mem::swap(&mut min_date_vec, &mut max_date_vec);
        }

        let (min_year, min_month, min_day) = (min_date_vec[0], min_date_vec[1], min_date_vec[2]);
        let (max_year, max_month, max_day) = (max_date_vec[0], max_date_vec[1], max_date_vec[2]);

        (min_year..max_year)
            .map(|year| if !is_leap_year(year) { 365 } else { 366 })
            .sum::<i32>()
            - month_day_calc(min_year, min_month, min_day)
            + month_day_calc(max_year, max_month, max_day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            0,
            Solution::days_between_dates("2019-06-29".to_owned(), "2019-06-29".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            1,
            Solution::days_between_dates("2019-06-29".to_owned(), "2019-06-28".to_owned())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            33485,
            Solution::days_between_dates("2074-09-12".to_owned(), "1983-01-08".to_owned())
        );
    }
}
