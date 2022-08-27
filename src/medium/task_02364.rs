// 2364. Count Number of Bad Pairs
// https://leetcode.com/problems/count-number-of-bad-pairs/

use crate::Solution;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        use std::collections::HashMap;
        let mut hm: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        let mut res = 0;
        nums.iter().zip(0..).for_each(|(&x, i)| {
            let delta = x - i;
            let e = hm.entry(delta).or_default();
            res += (i - *e) as i64;
            *e += 1;
        });
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_bad_pairs(vec![4, 1, 3, 3]), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::count_bad_pairs(vec![1, 2, 3, 4, 5]), 0);
    }
}
