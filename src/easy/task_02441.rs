// 2441. Largest Positive Integer That Exists With Its Negative
// https://leetcode.com/problems/largest-positive-integer-that-exists-with-its-negative/

use crate::Solution;

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut counter = [0_i32; 1001];
        let mut k = -1;
        nums.iter().for_each(|&x| {
            let n = &mut counter[x.unsigned_abs() as usize];
            let (sign_x, sign_n) = (x.signum(), (*n).signum());
            match sign_n {
                0 => *n = sign_x,
                _ if sign_n != sign_x => k = k.max(x.abs()),
                _ => (),
            }
        });
        k
    }

    // https://leetcode.com/problems/largest-positive-integer-that-exists-with-its-negative/discuss/2708093/PythonRust-fastest-(100)-using-two-pointers-binary-search-(with-detailed-comments)
    pub fn find_max_k_sort(mut nums: Vec<i32>) -> i32 {
        // [1] to use binary search, we need
        //     to prepare the list by sorting
        nums.sort();

        // [2] now we can move from the right
        //     and make binary search queries
        for i in (0..nums.len()).rev() {
            if nums.binary_search(&(-nums[i])).is_ok() {
                return nums[i];
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_max_k(vec![-1, 2, -3, 3]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_max_k(vec![-1, 10, 6, 7, -7, 1]), 7);
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::find_max_k(vec![-10, 8, 6, 7, -2, -3]), -1);
    }
}
