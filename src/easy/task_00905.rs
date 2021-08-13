// 905. Sort Array By Parity
// https://leetcode.com/problems/sort-array-by-parity/

use crate::Solution;

impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut am = a;
        let mut pos: usize = 0;
        let mut odd_here = false;

        for i in 0..am.len() {
            if am[i] % 2 == 1 {
                if !odd_here {
                    odd_here = true;
                    pos = i;
                }
            } else if odd_here {
                am.swap(pos, i);
                pos += 1;
            }
        }
        am
    }

    pub fn sort_array_by_parity_partition(a: Vec<i32>) -> Vec<i32> {
        let (mut even, mut odd): (Vec<i32>, Vec<i32>) = a.iter().partition(|&n| n % 2 == 0);
        even.append(&mut odd);
        even
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![2, 4, 3, 1],
            Solution::sort_array_by_parity(vec![3, 1, 2, 4])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![2, 4, 4, 6, 2, 3, 1, 7],
            Solution::sort_array_by_parity(vec![3, 1, 2, 4, 7, 4, 6, 2])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![2, 4, 4, 6, 2, 3, 1, 7],
            Solution::sort_array_by_parity_partition(vec![3, 1, 2, 4, 7, 4, 6, 2])
        );
    }
}
