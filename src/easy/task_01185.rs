// 1185. Day of the Week
// https://leetcode.com/problems/day-of-the-week/

use crate::Solution;

const WEEKDAYS: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let is_leap_year = |x: i32| -> bool { x % 4 == 0 && (x % 100 != 0 || x % 400 == 0) };

        let num_days = (1971..year)
            .map(|y| 365 + is_leap_year(y) as i32)
            .sum::<i32>()
            + (0..month - 1)
                .map(|m| months[m as usize] + (m == 1 && is_leap_year(year)) as i32)
                .sum::<i32>()
            + day;

        // 01/01/1971 - Friday
        let res_day = (4 + (num_days - 1)) % 7;
        WEEKDAYS[res_day as usize].to_string()
    }

    // https://leetcode.com/problems/day-of-the-week/discuss/695276/rust-Sakamoto's-algorithm
    pub fn day_of_the_week_sakamoto(day: i32, month: i32, year: i32) -> String {
        const SHIFT: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

        let year = year - (month < 3) as i32; // bool as i32 is false->0 || true->1
        WEEKDAYS[((year + year / 4 - year / 100 + year / 400 + SHIFT[(month - 1) as usize] + day)
            % 7) as usize]
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "Saturday".to_string(),
            Solution::day_of_the_week(31, 8, 2019)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!("Sunday".to_string(), Solution::day_of_the_week(18, 7, 1999));
    }

    #[test]
    fn test_3() {
        assert_eq!("Sunday".to_string(), Solution::day_of_the_week(15, 8, 1993));
    }

    #[test]
    fn test_4() {
        assert_eq!(
            "Thursday".to_string(),
            Solution::day_of_the_week(30, 9, 2100)
        );
    }
}
