// 1399. Count Largest Group
// https://leetcode.com/problems/count-largest-group/

use crate::Solution;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let sum_dig = |mut x| {
            let mut sum = 0;
            loop {
                sum += x % 10;
                x /= 10;
                if x < 10 {
                    break sum + x;
                }
            }
        };

        let mut arr_groups = [0; 37]; // 9999 - 36 max sum
        let mut max_group = 0;

        (1..=n as usize).map(sum_dig).for_each(|x| {
            arr_groups[x] += 1;
            max_group = max_group.max(arr_groups[x]);
        });

        arr_groups.iter().filter(|&&x| x == max_group).count() as i32
    }

    pub fn count_largest_group_dp(n: i32) -> i32 {
        let mut dp = [0; 10001];
        let mut arr_groups = [0; 37]; // 9999 - 36 max sum
        let mut max_group = 0;

        (1..=n as usize).for_each(|x| {
            let (quotient, reminder) = (x / 10, x % 10);
            let sum = reminder + dp[quotient];
            dp[x] = sum;
            arr_groups[sum] += 1;
            max_group = max_group.max(arr_groups[sum]);
        });
        arr_groups.iter().filter(|&&x| x == max_group).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::count_largest_group(13));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::count_largest_group(2));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::count_largest_group(9999));
    }

    #[test]
    fn test_4() {
        assert_eq!(1, Solution::count_largest_group_dp(9999));
    }
}
