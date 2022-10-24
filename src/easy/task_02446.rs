// 2446. Determine if Two Events Have Conflict
// https://leetcode.com/problems/determine-if-two-events-have-conflict/

use crate::Solution;

impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        let get_interval = |event: &[String]| -> (i32, i32) {
            (
                event[0][..2].parse::<i32>().unwrap() * 60 + event[0][3..].parse::<i32>().unwrap(),
                event[1][..2].parse::<i32>().unwrap() * 60 + event[1][3..].parse::<i32>().unwrap(),
            )
        };

        let (start_1, end_1) = get_interval(&event1);
        let (start_2, end_2) = get_interval(&event2);
        start_1 <= end_2 && start_2 <= end_1
    }

    // https://leetcode.com/problems/determine-if-two-events-have-conflict/discuss/2735340/Rust-or-One-liner
    pub fn have_conflict_oneliner(event1: Vec<String>, event2: Vec<String>) -> bool {
        event1[0] <= event2[1] && event1[1] >= event2[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::have_conflict(
            vec!["01:15".to_string(), "02:00".to_string()],
            vec!["02:00".to_string(), "03:00".to_string()]
        ));
    }

    #[test]
    fn test_2() {
        assert!(Solution::have_conflict(
            vec!["01:00".to_string(), "02:00".to_string()],
            vec!["01:20".to_string(), "03:00".to_string()]
        ));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::have_conflict(
            vec!["10:00".to_string(), "11:00".to_string()],
            vec!["14:00".to_string(), "15:00".to_string()]
        ));
    }
}
