// 2279. Maximum Bags With Full Capacity of Rocks
// https://leetcode.com/problems/maximum-bags-with-full-capacity-of-rocks/description/

use crate::Solution;

impl Solution {
    pub fn maximum_bags(mut capacity: Vec<i32>, rocks: Vec<i32>, mut additional_rocks: i32) -> i32 {
        capacity
            .iter_mut()
            .zip(rocks.iter())
            .for_each(|(c, &r)| *c -= r);
        capacity.sort_unstable();

        let mut res = 0;
        for &free_cap in &capacity {
            res += (free_cap <= additional_rocks) as i32;
            additional_rocks -= free_cap;
            if additional_rocks <= 0 {
                break;
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
        assert_eq!(
            3,
            Solution::maximum_bags(vec![2, 3, 4, 5], vec![1, 2, 4, 4], 2)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::maximum_bags(vec![10, 2, 2], vec![2, 2, 0], 100)
        );
    }
}
