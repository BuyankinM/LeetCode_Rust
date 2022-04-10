// 2231. Largest Number After Digit Swaps by Parity
// https://leetcode.com/problems/largest-number-after-digit-swaps-by-parity/

use crate::Solution;

impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        use std::collections::BinaryHeap;

        let mut num = num as usize;
        let mut evens_odds = [BinaryHeap::new(), BinaryHeap::new()];
        let mut digits = Vec::new();

        while num > 0 {
            let dig = num % 10;
            evens_odds[dig % 2].push(dig as i32);
            digits.push(dig);
            num /= 10;
        }

        digits
            .iter()
            .rev()
            .fold(0, |res, &dig| 10 * res + evens_odds[dig % 2].pop().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3412, Solution::largest_integer(1234));
    }

    #[test]
    fn test_2() {
        assert_eq!(87655, Solution::largest_integer(65875));
    }

    #[test]
    fn test_3() {
        assert_eq!(889756620, Solution::largest_integer(687598206));
    }
}
