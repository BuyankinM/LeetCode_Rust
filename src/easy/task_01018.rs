// 1018. Binary Prefix Divisible By 5
// https://leetcode.com/problems/binary-prefix-divisible-by-5/

use crate::Solution;

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut res = Vec::with_capacity(nums.len());
        let mut val = 0;

        for &num in &nums {
            val = (val << 1 | num) % 5;
            res.push(val == 0);
        }
        res
    }

    pub fn prefixes_div_by5_one_liner(nums: Vec<i32>) -> Vec<bool> {
        nums.iter()
            .scan(0, |acc, num| {
                *acc = (*acc << 1 | *num) % 5;
                Some(*acc == 0)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![true, false, false],
            Solution::prefixes_div_by5(vec![0, 1, 1])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![false, false, false],
            Solution::prefixes_div_by5(vec![1, 1, 1])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, true, false, false, true, true,
                true, true, false
            ],
            Solution::prefixes_div_by5_one_liner(vec![
                1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0,
                0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1
            ])
        );
    }
}
