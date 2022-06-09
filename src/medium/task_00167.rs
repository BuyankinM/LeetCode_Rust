// 167. Two Sum II - Input Array Is Sorted
// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/

use crate::Solution;

impl Solution {
    pub fn two_sum_ii(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut end = numbers.len() - 1;

        if target > 0 {
            if let Ok(i) = numbers.binary_search(&target) {
                end = i;
            }
        }

        loop {
            let val = target - numbers[end];
            match numbers[..end].binary_search(&val) {
                Ok(i) => break vec![i as i32 + 1, end as i32 + 1],
                _ => end -= 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![1, 2], Solution::two_sum_ii(vec![2, 7, 11, 15], 9));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![1, 3], Solution::two_sum_ii(vec![2, 3, 4], 6));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![1, 2], Solution::two_sum_ii(vec![-1, 0], -1));
    }

    #[test]
    fn test_4() {
        assert_eq!(vec![1, 2], Solution::two_sum_ii(vec![0, 0, 3, 4], 0));
    }
}
