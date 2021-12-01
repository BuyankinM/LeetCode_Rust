// 198. House Robber
// https://leetcode.com/problems/house-robber/

use crate::Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            1 => nums[0],
            _ => {
                let l = nums.len();
                let mut max_money = Vec::with_capacity(l);
                for (i, &m) in nums.iter().enumerate() {
                    let prev_max = match i {
                        0 | 1 => 0,
                        2 => max_money[0],
                        _ => max_money[i - 2].max(max_money[i - 3]),
                    };
                    max_money.push(m + prev_max)
                }
                max_money[l - 1].max(max_money[l - 2])
            }
        }
    }

    // https://leetcode.com/problems/house-robber/discuss/224675/Rust-0-ms
    pub fn rob_short(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, 0), |(prev_prev, prev), x| {
                (prev, prev.max(prev_prev + x))
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(21, Solution::rob(vec![2, 7, 9, 3, 1, 10]));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(7, Solution::rob(vec![2, 7]));
    }
}
