// 2367. Number of Arithmetic Triplets
// https://leetcode.com/problems/number-of-arithmetic-triplets/

use crate::Solution;

const N: usize = 200;

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut counter = [0; N + 1];
        let (mut res, mut prev) = (0, -1);

        for &n in &nums {
            if n == prev {
                continue;
            }

            (n as usize..)
                .step_by(diff as usize)
                .take(3)
                .filter(|&i| i <= N)
                .for_each(|idx| counter[idx] += 1);

            if counter[n as usize] == 3 {
                res += 1;
            }

            prev = n;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::arithmetic_triplets(vec![0, 1, 4, 6, 7, 10], 3), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::arithmetic_triplets(vec![4, 5, 6, 7, 8, 9], 2), 2);
    }
}
