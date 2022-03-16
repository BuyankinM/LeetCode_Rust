// 495. Teemo Attacking
// https://leetcode.com/problems/teemo-attacking/

use crate::Solution;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        if duration == 0 {
            return 0;
        }

        let mut res = 0;
        let mut prev_end = -1;

        for &x in &time_series {
            let end = x + duration - 1;
            if x > prev_end {
                res += duration;
            } else if end > prev_end {
                res += end - prev_end;
            }
            prev_end = end;
        }
        res
    }

    pub fn find_poisoned_duration_func(time_series: Vec<i32>, duration: i32) -> i32 {
        time_series
            .windows(2)
            .map(|p| duration.min(p[1] - p[0]))
            .sum::<i32>()
            + duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::find_poisoned_duration(vec![1, 4], 2));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::find_poisoned_duration(vec![1, 4], 0));
    }

    #[test]
    fn test_3() {
        assert_eq!(3, Solution::find_poisoned_duration(vec![1, 2], 2));
    }

    #[test]
    fn test_4() {
        assert_eq!(1, Solution::find_poisoned_duration(vec![1, 1], 1));
    }
}
