// 2511. Maximum Enemy Forts That Can Be Captured
// https://leetcode.com/problems/maximum-enemy-forts-that-can-be-captured/

use crate::Solution;

impl Solution {
    pub fn capture_forts(forts: Vec<i32>) -> i32 {
        let (mut result, mut counter, mut prev_fort) = (0, 0, 0);
        for &x in &forts {
            match x {
                0 => counter += 1,
                _ => {
                    if prev_fort == -x {
                        result = result.max(counter);
                    }
                    counter = 0;
                    prev_fort = x;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::capture_forts(vec![1, 0, 0, -1, 0, 0, 0, 0, 1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::capture_forts(vec![0, 0, 1, -1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            4,
            Solution::capture_forts(vec![1, -1, 0, 0, -1, 0, 0, 0, 0, 1])
        );
    }
}
