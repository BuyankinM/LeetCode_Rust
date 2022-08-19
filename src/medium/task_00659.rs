// 659. Split Array into Consecutive Subsequences
// https://leetcode.com/problems/split-array-into-consecutive-subsequences/

use crate::Solution;

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let idx = |x: &i32| (*x + 1000) as usize;
        let mut counter = [0; 2001];
        let mut sequences = [0; 2001];
        nums.iter().for_each(|x| counter[idx(x)] += 1);

        for x in nums.iter().map(idx) {
            match counter[x] > 0 {
                true => counter[x] -= 1,
                false => continue,
            }

            if sequences[x] > 0 {
                // update sequences
                sequences[x] -= 1;
                sequences[x + 1] += 1;
            } else if counter[x + 1] > 0 && counter[x + 2] > 0 {
                // check for new sequence
                counter[x + 1] -= 1;
                counter[x + 2] -= 1;
                sequences[x + 3] += 1;
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_possible(vec![1, 2, 3, 3, 4, 5]));
    }

    #[test]
    fn test_2() {
        assert!(Solution::is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::is_possible(vec![1, 2, 3, 4, 4, 5]));
    }
}
