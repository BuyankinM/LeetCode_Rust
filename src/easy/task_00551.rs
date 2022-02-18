// 551. Student Attendance Record I
// https://leetcode.com/problems/student-attendance-record-i/

use crate::Solution;

impl Solution {
    pub fn check_record(s: String) -> bool {
        let (mut absent, mut late) = (0, 0);
        let mut prev_c = ' ';
        for c in s.chars() {
            match c {
                'A' => absent += 1,
                'L' => late = if prev_c == c { late + 1 } else { 1 },
                _ => (),
            }
            if absent == 2 || late == 3 {
                return false;
            }
            prev_c = c;
        }
        true
    }

    // https://leetcode.com/problems/student-attendance-record-i/discuss/849586/Rust-cheapest-and-best
    pub fn check_record_func(s: String) -> bool {
        s.find("LLL")
            .map(|_| false)
            .unwrap_or(s.chars().filter(|&c| c == 'A').count() < 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::check_record("PPALLP".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::check_record("PPALLL".to_string()));
    }
}
