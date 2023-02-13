// 2562. Find the Array Concatenation Value
// https://leetcode.com/problems/find-the-array-concatenation-value/

use crate::Solution;

impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let mut result = 0;
        let mut it = nums.into_iter();
        loop {
            match it.next() {
                Some(first) => {
                    result += match it.next_back() {
                        Some(last) => {
                            let p = 1 + (last as f64).log10() as u32;
                            (first as i64) * 10_i64.pow(p) + last as i64
                        }
                        None => first as i64,
                    }
                }
                None => break result,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(596, Solution::find_the_array_conc_val(vec![7, 52, 2, 4]));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            673,
            Solution::find_the_array_conc_val(vec![5, 14, 13, 8, 12])
        );
    }
}
