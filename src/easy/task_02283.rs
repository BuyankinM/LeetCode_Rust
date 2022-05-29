// 2283. Check if Number Has Equal Digit Count and Digit Value
// https://leetcode.com/problems/check-if-number-has-equal-digit-count-and-digit-value/

use crate::Solution;

impl Solution {
    pub fn digit_count(num: String) -> bool {
        let mut counter = [0; 10];
        let num_it = num.bytes().map(|b| (b - b'0') as usize);
        num_it.clone().for_each(|idx| counter[idx] += 1);
        num_it.enumerate().all(|(i, idx)| counter[i] == idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::digit_count("1210".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::digit_count("030".to_string()));
    }
}
