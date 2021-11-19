// 860. Lemonade Change
// https://leetcode.com/problems/lemonade-change/

use crate::Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut num_5, mut num_10) = (0, 0);
        for &bill in &bills {
            if bill == 5 {
                num_5 += 1;
            } else if bill == 10 {
                match num_5 >= 1 {
                    true => num_5 -= 1,
                    false => return false,
                };
                num_10 += 1;
            } else if num_10 >= 1 && num_5 >= 1 {
                num_10 -= 1;
                num_5 -= 1;
            } else if num_5 >= 3 {
                num_5 -= 3;
            } else {
                return false;
            }
        }
        true
    }

    pub fn lemonade_change_short(bills: Vec<i32>) -> bool {
        let (mut num_5, mut num_10) = (0, 0);
        for bill in bills {
            match bill {
                5 => num_5 += 1,
                10 => {
                    num_5 -= 1;
                    num_10 += 1;
                }
                20 if num_10 >= 1 => {
                    num_5 -= 1;
                    num_10 -= 1;
                }
                20 => num_5 -= 3,
                _ => panic!("Invalid bill"),
            }

            if num_5 < 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]));
    }

    #[test]
    fn test_2() {
        assert!(Solution::lemonade_change_short(vec![5, 5, 5, 10, 20]));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::lemonade_change_short(vec![5, 5, 10, 10, 20]));
    }

    #[test]
    fn test_4() {
        assert!(Solution::lemonade_change_short(vec![5, 5, 10]));
    }
}
