// 374. Guess Number Higher or Lower
// https://leetcode.com/problems/guess-number-higher-or-lower/

use crate::Solution;

// Forward declaration of guess API.
// @param  num   your guess
// @return       -1 if num is lower than the guess number
//                1 if num is higher than the guess number
//                otherwise return 0
// unsafe fn guess(num: i32) -> i32 {}

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        fn guess(n: i32) -> i32 {
            let k = 1000;
            (k - n).signum()
        }

        let (mut low, mut high) = (1, n);
        let mut mid: i32;
        loop {
            mid = low + (high - low) / 2;
            match guess(mid) {
                0 => break,
                -1 => high = mid - 1,
                _ => low = mid + 1,
            }
        }
        mid
    }
}
