// 1629. Slowest Key
// https://leetcode.com/problems/slowest-key/

use crate::Solution;

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let (mut max_time, mut max_key) = (0, ' ');
        release_times
            .iter()
            .zip(keys_pressed.chars())
            .fold(0, |t, (&ti, c)| {
                let delay = ti - t;
                if delay > max_time || delay == max_time && c > max_key {
                    max_key = c;
                    max_time = delay;
                }
                ti
            });
        max_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            'c',
            Solution::slowest_key(vec![9, 29, 49, 50], "cbcd".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            'a',
            Solution::slowest_key(vec![12, 23, 36, 46, 62], "spuda".to_owned())
        );
    }
}
