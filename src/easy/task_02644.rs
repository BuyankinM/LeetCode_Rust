// 2644. Find the Maximum Divisibility Score
// https://leetcode.com/problems/find-the-maximum-divisibility-score/

use crate::Solution;

impl Solution {
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        let (mut min_div, mut max_score) = (0, -1);
        for div in divisors {
            let score = nums.iter().filter(|&x| x % div == 0).count() as i32;
            if score > max_score || score == max_score && div < min_div {
                min_div = div;
                max_score = score;
            }
        }
        min_div
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::max_div_score(vec![4, 7, 9, 3, 9], vec![5, 2, 3])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            5,
            Solution::max_div_score(vec![20, 14, 21, 10], vec![5, 7, 5])
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(10, Solution::max_div_score(vec![12], vec![10, 16]));
    }
}
