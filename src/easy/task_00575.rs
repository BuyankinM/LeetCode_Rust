// 575. Distribute Candies
// https://leetcode.com/problems/distribute-candies/

use crate::Solution;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let set: std::collections::HashSet<_> = candy_type.iter().collect();
        let cl = candy_type.len() / 2;
        cl.min(set.len()) as i32
    }

    pub fn distribute_candies_one_liner(candy_type: Vec<i32>) -> i32 {
        candy_type
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            .min(candy_type.len() / 2) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::distribute_candies(vec![1, 1, 2, 3]));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::distribute_candies_one_liner(vec![6, 6, 6, 6]));
    }
}
