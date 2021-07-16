// 69. Sqrt(x)
// https://leetcode.com/problems/sqrtx/

use crate::Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        use std::cmp::Ordering;
        if x <= 1 {
            return x;
        }

        let (mut low, mut high) = (0, x / 2 + 1);
        let mut result = 0;

        while low <= high {
            let mid = low + (high - low) / 2;
            match mid.cmp(&(x / mid)) {
                Ordering::Greater => high = mid - 1,
                Ordering::Less => {
                    low = mid + 1;
                    result = mid
                }
                Ordering::Equal => return mid,
            }
        }
        result
    }

    // (n+1)² = n² + 2n + 1
    pub fn my_sqrt_formula(x: i32) -> i32 {
        let mut n = 0;
        let mut x = x - 1;
        while x >= 0 {
            n += 1;
            x -= 2 * n + 1;
        }
        n
    }

    pub fn my_sqrt_newton(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let x = x as usize;
        let mut n = x;
        while n > x / n {
            n = (n + x / n) / 2;
        }
        n as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::my_sqrt(4));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::my_sqrt(8));
    }

    #[test]
    fn test_3() {
        assert_eq!(3, Solution::my_sqrt(9));
    }

    #[test]
    fn test_4() {
        assert_eq!(46339, Solution::my_sqrt(2147395599));
    }

    #[test]
    fn test_5() {
        assert_eq!(46340, Solution::my_sqrt(2147483647));
    }

    #[test]
    fn test_6() {
        assert_eq!(10, Solution::my_sqrt(100));
    }

    #[test]
    fn test_7() {
        assert_eq!(3, Solution::my_sqrt(15));
    }
}
