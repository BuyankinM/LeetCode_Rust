// 2614. Prime In Diagonal
// https://leetcode.com/problems/prime-in-diagonal/

use crate::Solution;

impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        fn is_prime(x: i32) -> bool {
            match x {
                1 => return false,
                2 => return true,
                _ if x % 2 == 0 => return false,
                _ => {
                    for y in (3..((x as f32).sqrt() as i32 + 1)).step_by(2) {
                        if x % y == 0 {
                            return false;
                        }
                    }
                }
            }
            true
        }

        let l = nums.len();
        let mut prime = 0;
        for (i, row) in nums.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                if x > prime && (i == j || j == l - i - 1) && is_prime(x) {
                    prime = x;
                }
            }
        }
        prime
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            11,
            Solution::diagonal_prime(vec![vec![1, 2, 3], vec![5, 6, 7], vec![9, 10, 11]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            17,
            Solution::diagonal_prime(vec![vec![1, 2, 3], vec![5, 17, 7], vec![9, 11, 10]])
        );
    }
}
