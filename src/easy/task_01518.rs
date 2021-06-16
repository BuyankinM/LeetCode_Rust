// 1518. Water Bottles
// https://leetcode.com/problems/water-bottles/

use crate::Solution;

impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut res = num_bottles;
        while num_bottles >= num_exchange {
            res += num_bottles / num_exchange;
            num_bottles = num_bottles / num_exchange + num_bottles % num_exchange;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::num_water_bottles(5, 5));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::num_water_bottles(2, 3));
    }
}
