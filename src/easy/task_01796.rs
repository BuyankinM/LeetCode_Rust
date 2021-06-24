// 1796. Second Largest Digit in a String
// https://leetcode.com/problems/second-largest-digit-in-a-string/

use crate::Solution;

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut ar = [false; 10];

        s.as_bytes()
            .iter()
            .filter(|x| x.is_ascii_digit())
            .map(|x| x - b'0')
            .for_each(|x| ar[x as usize] = true);

        ar.iter()
            .enumerate()
            .filter_map(|(ind, val)| if *val { Some(ind as i32) } else { None })
            .rev()
            .nth(1)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::second_highest("dfa12321afd".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(-1, Solution::second_highest("abc1111".to_owned()));
    }
}
