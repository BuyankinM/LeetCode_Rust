// 414. Third Maximum Number
// https://leetcode.com/problems/third-maximum-number/

use crate::Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut m1 = std::i32::MIN;
        let mut m2 = std::i32::MIN;
        let mut m3 = std::i32::MIN;
        let mut get_m = 0;

        for n in nums {
            if (n == m1 && get_m >= 1) || (n == m2 && get_m >= 2) {
                continue;
            }

            if (n > m1) || (n == m1 && get_m == 0) {
                m3 = std::mem::replace(&mut m2, m1);
                m1 = n;
                get_m += 1;
            } else if (n > m2) || (n == m2 && get_m == 1) {
                m3 = std::mem::replace(&mut m2, n);
                get_m += 1;
            } else if (n > m3) || (n == m3 && get_m == 2) {
                m3 = n;
                get_m += 1;
            }
        }

        match get_m >= 3 {
            true => m3,
            false => m1,
        }
    }

    pub fn third_max_option(nums: Vec<i32>) -> i32 {
        let mut first = None;
        let mut second = None;
        let mut third = None;
        let mut tmp;

        for n in nums {
            let n = Some(n);
            if n > first {
                tmp = second;
                second = first;
                first = n;
                third = tmp;
            } else if n < first && n > second {
                tmp = second;
                second = n;
                third = tmp;
            } else if n < second && n >= third {
                third = n;
            }
        }
        third.unwrap_or_else(|| first.unwrap())
    }

    pub fn third_max_sort(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.dedup();
        nums[nums.len() - if nums.len() > 2 { 3 } else { 1 }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::third_max(vec![3, 2, 1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::third_max(vec![1, 2]));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::third_max(vec![2, 2, 3, 1]));
    }

    #[test]
    fn test_4() {
        assert_eq!(1, Solution::third_max_option(vec![3, 2, 1]));
    }

    #[test]
    fn test_5() {
        assert_eq!(2, Solution::third_max_option(vec![1, 2]));
    }

    #[test]
    fn test_6() {
        assert_eq!(1, Solution::third_max_option(vec![2, 2, 3, 1]));
    }
}
