// 2409. Count Days Spent Together
// https://leetcode.com/problems/count-days-spent-together/

use crate::Solution;

const MONTHS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let parse_num_date = |s: String| -> i32 {
            let mut it = s.split('-').map(|mds| mds.parse::<i32>().unwrap());
            let (month, day) = (it.next().unwrap(), it.next().unwrap());
            MONTHS[..(month - 1) as usize].iter().sum::<i32>() + day
        };

        let arrive_alice_day = parse_num_date(arrive_alice);
        let leave_alice_day = parse_num_date(leave_alice);
        let arrive_bob_day = parse_num_date(arrive_bob);
        let leave_bob_day = parse_num_date(leave_bob);

        let min_day = arrive_alice_day.max(arrive_bob_day);
        let max_day = leave_alice_day.min(leave_bob_day);
        0.max(max_day - min_day + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::count_days_together(
                "08-15".to_string(),
                "08-18".to_string(),
                "08-16".to_string(),
                "08-19".to_string()
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            0,
            Solution::count_days_together(
                "10-01".to_string(),
                "10-31".to_string(),
                "11-01".to_string(),
                "12-31".to_string()
            )
        );
    }
}
