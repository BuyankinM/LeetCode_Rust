// 303. Range Sum Query - Immutable
// https://leetcode.com/problems/range-sum-query-immutable/

use crate::Solution;

struct NumArray {
    acc_sum: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let acc_sum = nums
            .into_iter()
            .scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            })
            .collect();

        NumArray { acc_sum }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let start = left as usize;
        let end = right as usize;
        match start {
            0 => self.acc_sum[end],
            _ => self.acc_sum[end] - self.acc_sum[start - 1],
        }
    }
}
