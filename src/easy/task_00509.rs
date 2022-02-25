// 509. Fibonacci Number
// https://leetcode.com/problems/fibonacci-number/

use crate::Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        match n < 2 {
            true => n,
            false => {
                let mut arr = [0, 1];
                (2..=n as usize).for_each(|x| arr[x % 2] = arr[0] + arr[1]);
                arr[(n % 2) as usize]
            }
        }
    }

    // https://leetcode.com/problems/fibonacci-number/discuss/260170/Rust-basic-dynamic-programming-0ms
    pub fn fib_short_func(n: i32) -> i32 {
        (0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::fib(2));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::fib(3));
    }

    #[test]
    fn test_3() {
        assert_eq!(3, Solution::fib(4));
    }

    #[test]
    fn test_4() {
        assert_eq!(75025, Solution::fib(25));
    }
}
