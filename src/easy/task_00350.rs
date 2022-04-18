// 350. Intersection of Two Arrays II
// https://leetcode.com/problems/intersection-of-two-arrays-ii/

use crate::Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::iter::repeat;

        let mut counter_1 = [0; 1001];
        let mut counter_2 = [0; 1001];
        nums1.iter().for_each(|x| counter_1[*x as usize] += 1);
        nums2.iter().for_each(|x| counter_2[*x as usize] += 1);

        counter_1
            .iter()
            .zip(counter_2.iter())
            .enumerate()
            .filter(|(_, (&x, &y))| x > 0 && y > 0)
            .flat_map(|(i, (&x, &y))| repeat(i as i32).take(x.min(y) as usize))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2, 2]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            vec![4, 9]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2, 1]),
            vec![1, 2, 2]
        );
    }
}
