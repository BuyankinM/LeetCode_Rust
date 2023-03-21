// 2348. Number of Zero-Filled Subarrays
// https://leetcode.com/problems/number-of-zero-filled-subarrays/description/

use crate::Solution;

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut res = 0;
        let mut n = 0;
        for &x in &nums {
            match x == 0 {
                true => n += 1,
                false => {
                    res += n * (n + 1) / 2;
                    n = 0
                }
            }
        }
        res + n * (n + 1) / 2
    }

    // https://leetcode.com/problems/number-of-zero-filled-subarrays/solutions/3322670/easy-to-understand-solution-with-explanation/
    pub fn zero_filled_subarray_optimal(nums: Vec<i32>) -> i64 {
        let mut count = 0;
        let mut subcount = 0;
        for num in nums {
            if num != 0 {
                subcount = 0;
                continue;
            }
            subcount += 1;
            count += subcount;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            6,
            Solution::zero_filled_subarray(vec![1, 3, 0, 0, 2, 0, 0, 4])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(9, Solution::zero_filled_subarray(vec![0, 0, 0, 2, 0, 0]));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::zero_filled_subarray(vec![2, 10, 2019]));
    }
}
