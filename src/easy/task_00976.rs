// 976. Largest Perimeter Triangle
// https://leetcode.com/problems/largest-perimeter-triangle/

use crate::Solution;

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by(|a, b| b.cmp(a));
        match nums.windows(3).find(|l| l[0] < l[1] + l[2]) {
            Some(l) => l.iter().sum(),
            None => 0,
        }
    }

    pub fn largest_perimeter_heap(nums: Vec<i32>) -> i32 {
        let mut heap = std::collections::BinaryHeap::from(nums);
        let (mut b, mut c) = (-1, -1);
        while let Some(a) = heap.pop() {
            if a > 0 && b > 0 && c > 0 && c < b + a {
                return a + b + c;
            }
            c = b;
            b = a;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::largest_perimeter(vec![2, 1, 2]));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::largest_perimeter(vec![1, 2, 1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(10, Solution::largest_perimeter(vec![3, 2, 3, 4]));
    }

    #[test]
    fn test_4() {
        assert_eq!(8, Solution::largest_perimeter(vec![3, 6, 2, 3]));
    }

    #[test]
    fn test_5() {
        assert_eq!(8, Solution::largest_perimeter_heap(vec![3, 6, 2, 3]));
    }
}
