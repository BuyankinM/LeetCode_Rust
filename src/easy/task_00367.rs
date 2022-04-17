// 367. Valid Perfect Square
// https://leetcode.com/problems/valid-perfect-square/

use crate::Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        use std::cmp::Ordering;
        let (mut low, mut high) = (0, num);
        while low <= high {
            let mid = low + (high - low) / 2;
            match mid.checked_mul(mid) {
                Some(square) => match square.cmp(&num) {
                    Ordering::Equal => return true,
                    Ordering::Less => low = mid + 1,
                    Ordering::Greater => high = mid - 1,
                },
                None => high = mid - 1,
            }
        }
        false
    }

    pub fn is_perfect_square_func(num: i32) -> bool {
        (1..=num)
            .take_while(|&x| x.checked_mul(x).map_or(false, |res| res <= num))
            .last()
            .map_or(false, |x| x * x == num)
    }

    // https://leetcode.com/problems/valid-perfect-square/discuss/735543/Rust-Solutions
    pub fn is_perfect_square_1357(num: i32) -> bool {
        let mut num = num;
        let mut i = 1;
        while num > 0 {
            num -= i;
            i += 2;
        }
        num == 0
    }

    // https://leetcode.com/problems/valid-perfect-square/discuss/735543/Rust-Solutions
    pub fn is_perfect_square_newton(num: i32) -> bool {
        let mut i = num as i64;
        let num = num as i64;

        while i > num / i {
            i = (i + num / i) / 2;
        }

        i * i == num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_perfect_square(1));
    }

    #[test]
    fn test_2() {
        assert!(Solution::is_perfect_square(16));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::is_perfect_square(2147483647));
    }

    #[test]
    fn test_4() {
        assert!(Solution::is_perfect_square(808201));
    }
}
