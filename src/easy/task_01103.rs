// 1103. Distribute Candies to People
// https://leetcode.com/problems/distribute-candies-to-people/

use crate::Solution;

impl Solution {
    pub fn distribute_candies_new(mut candies: i32, num_people: i32) -> Vec<i32> {
        let l = num_people as usize;
        let mut res = vec![0; l];
        let mut c = 1;

        while candies > 0 {
            res[(c - 1) % l] += c as i32;
            candies -= c as i32;
            c += 1;
        }

        res[(c - 2) % l] += candies; // correct last value
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![1, 2, 3, 1], Solution::distribute_candies_new(7, 4));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![5, 2, 3], Solution::distribute_candies_new(10, 3));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![142309, 142511, 142713, 142915, 143117, 143319, 143116],
            Solution::distribute_candies_new(1_000_000, 7)
        );
    }
}
