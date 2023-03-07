// 2582. Pass the Pillow
// https://leetcode.com/problems/pass-the-pillow/

use crate::Solution;

impl Solution {
    pub fn pass_the_pillow(n: i32, mut time: i32) -> i32 {
        let full_cycle = (n - 1) * 2;
        time = time % full_cycle + 1; // drop full cycle
        time.min(n * 2 - time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::pass_the_pillow(4, 5))
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::pass_the_pillow(3, 2))
    }

    #[test]
    fn test_3() {
        assert_eq!(5, Solution::pass_the_pillow(18, 38))
    }
}
