// 279. Perfect Squares
// https://leetcode.com/problems/perfect-squares/

use crate::Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let nu = n as usize;
        let mut dp = vec![std::i32::MAX; nu + 1];
        dp[0] = 0;

        for i in 1..(nu + 1) {
            let mut j = 1;
            while j * j <= i {
                dp[i] = dp[i].min(dp[i - j * j] + 1);
                j += 1;
            }
        }
        dp[nu]
    }

    // https://leetcode.com/problems/perfect-squares/discuss/71488/Summary-of-4-different-solutions-(BFS-DP-static-DP-and-mathematics)
    // https://en.wikipedia.org/wiki/Lagrange%27s_four-square_theorem
    // Lagrange's four-square theorem
    // every natural number can be represented as the sum of four integer squares
    // p = a0^2 + a1^2 + a2^2 + a3^2
    // https://en.wikipedia.org/wiki/Legendre%27s_three-square_theorem
    // Legendre's three-square theorem
    // n = x^2 + y^2 + z^2
    // if and only if n is not of the form n = 4^a(8b + 7) for nonnegative integers a and b.
    pub fn num_squares_math(mut n: i32) -> i32 {
        fn is_square(n: i32) -> bool {
            let sqrt_n = (n as f64).sqrt() as i32;
            sqrt_n * sqrt_n == n
        }

        if is_square(n) {
            return 1;
        }

        // Legendre's three-square theorem
        // if n != 4^a(8b + 7)
        while n % 4 == 0 {
            n >>= 2;
        }
        if n % 8 == 7 {
            return 4;
        }

        // check 2 square
        // n/=4 does not effect result
        // proof:
        // n = x^2 + y^2 = 4^a * b
        // b = (x^2 + y^2) / (4^a)
        //   = x^2/(4^a) + y^2/(4^a)
        //   = (x/(2^a))^2 + (y/(2^a))^2
        let mut i = 1;
        while i * i < n {
            if is_square(n - i * i) {
                return 2;
            }
            i += 1;
        }
        3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::num_squares(12));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, Solution::num_squares(1472));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::num_squares(100));
    }
}
