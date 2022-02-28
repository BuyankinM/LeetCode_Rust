// 507. Perfect Number
// https://leetcode.com/problems/perfect-number/

use crate::Solution;

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }

        let mut s = 1;
        for x in 2..=(num as f32).sqrt() as i32 {
            if num % x == 0 {
                s += x + if x * x < num { num / x } else { 0 };
                if s > num {
                    return false;
                }
            }
        }
        s == num
    }

    // https://leetcode.com/problems/perfect-number/discuss/380072/Rust-single-expression-solution
    pub fn check_perfect_number_func(num: i32) -> bool {
        num >= 2
            && (1..)
                .take_while(|x| x * x <= num)
                .filter(|x| num % x == 0)
                .fold(0, |acc, x| {
                    acc + {
                        if x * x == num || x == 1 {
                            x
                        } else {
                            x + num / x
                        }
                    }
                })
                == num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::check_perfect_number(28));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::check_perfect_number(1));
    }

    #[test]
    fn test_3() {
        assert!(Solution::check_perfect_number(33550336));
    }
}
