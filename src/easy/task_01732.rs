// 1732. Find the Highest Altitude
// https://leetcode.com/problems/find-the-highest-altitude/

use crate::Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        gain.iter()
            .fold((0, 0), |(mut alt, mut max_alt), &x| {
                alt += x;
                max_alt = max_alt.max(alt);
                (alt, max_alt)
            })
            .1
    }

    pub fn largest_altitude_short(gain: Vec<i32>) -> i32 {
        gain.iter()
            .fold((0, 0), |(max_alt, alt), &x| (max_alt.max(alt + x), alt + x))
            .0
    }

    pub fn largest_altitude_scan(gain: Vec<i32>) -> i32 {
        std::iter::once(0)
            .chain(gain.into_iter())
            .scan(0, |h, g| {
                *h += g;
                Some(*h)
            })
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::largest_altitude(vec![-5, 1, 5, 0, -7]));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]));
    }
}
