// 2455. Average Value of Even Numbers That Are Divisible by Three
// https://leetcode.com/problems/average-value-of-even-numbers-that-are-divisible-by-three/

use crate::Solution;

impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let (sum, n) = nums
            .iter()
            .fold((0, 0), |(sum, n), &x| match x % 2 == 0 && x % 3 == 0 {
                true => (sum + x, n + 1),
                false => (sum, n),
            });
        sum / 1.max(n)
    }

    // https://leetcode.com/problems/average-value-of-even-numbers-that-are-divisible-by-three/discuss/2758403/Rust-or-Filter-or-With-Comments
    pub fn average_value_vec(nums: Vec<i32>) -> i32 {
        let div = nums
            .into_iter()
            .filter(|n| *n % 2 == 0 && *n % 3 == 0)
            .collect::<Vec<_>>();
        let n = div.len() as i32;
        if n == 0 {
            0
        } else {
            div.into_iter().sum::<i32>() / n
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(9, Solution::average_value(vec![1, 3, 6, 10, 12, 15]));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::average_value(vec![1, 2, 4, 7, 10]));
    }
}
