// 2442. Count Number of Distinct Integers After Reverse Operations
// https://leetcode.com/problems/count-number-of-distinct-integers-after-reverse-operations/

use crate::Solution;

impl Solution {
    pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
        let rev = |mut n: i32| {
            let mut res = 0;
            while n > 0 {
                res = 10 * res + n % 10;
                n /= 10;
            }
            res
        };

        let mut set = std::collections::HashSet::with_capacity(nums.len() * 2);
        nums.iter().for_each(|&x| {
            set.insert(x);
            set.insert(rev(x));
        });
        set.len() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::count_distinct_integers(vec![1, 13, 10, 12, 31]),
            6
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::count_distinct_integers(vec![2, 2, 2]), 1);
    }
}
