// 1995. Count Special Quadruplets
// https://leetcode.com/problems/count-special-quadruplets/

use crate::Solution;

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut res = 0;
        for (ia, a) in nums[..l - 3].iter().enumerate() {
            for (ib, b) in nums[..l - 2].iter().enumerate().skip(ia + 1) {
                for (ic, c) in nums[..l - 1].iter().enumerate().skip(ib + 1) {
                    let s = a + b + c;
                    for &d in &nums[ic + 1..] {
                        if d == s {
                            res += 1
                        }
                    }
                }
            }
        }
        res
    }

    pub fn count_quadruplets_N3(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut res = 0;
        let mut cnt = [0; 101];

        for (ic, &c) in nums[2..].iter().rev().enumerate() {
            cnt[c as usize] += 1;
            for (ia, &a) in nums[..l - ic - 2].iter().enumerate() {
                for &b in nums[ia + 1..l - ic - 1].iter() {
                    let s = (a + b + c) as usize;
                    if s <= 100 {
                        res += cnt[s];
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::count_quadruplets(vec![1, 2, 3, 6]))
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::count_quadruplets(vec![3, 3, 6, 4, 5]))
    }

    #[test]
    fn test_3() {
        assert_eq!(4, Solution::count_quadruplets(vec![1, 1, 1, 3, 5]))
    }

    #[test]
    fn test_4() {
        assert_eq!(4, Solution::count_quadruplets_N3(vec![1, 1, 1, 3, 5]))
    }

    #[test]
    fn test_5() {
        assert_eq!(
            0,
            Solution::count_quadruplets_N3(vec![23, 39, 3, 35, 40, 37])
        )
    }

    #[test]
    fn test_6() {
        assert_eq!(2, Solution::count_quadruplets_N3(vec![9, 6, 8, 23, 39, 23]))
    }
}
