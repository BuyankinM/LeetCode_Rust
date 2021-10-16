// 1137. N-th Tribonacci Number
// https://leetcode.com/problems/n-th-tribonacci-number/

use crate::Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let ind = n as usize;
        let mut v = [0, 1, 1];
        match ind < 3 {
            true => v[ind],
            false => {
                (3..ind).for_each(|i| {
                    v[i % 3] = v.iter().sum();
                });
                v[ind % 3]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::tribonacci(0));
    }

    #[test]
    fn test_2() {
        assert_eq!(223_317, Solution::tribonacci(25));
    }

    #[test]
    fn test_3() {
        assert_eq!(181_997_601, Solution::tribonacci(36));
    }
}
