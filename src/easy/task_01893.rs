// 1893. Check if All the Integers in a Range Are Covered
// https://leetcode.com/problems/check-if-all-the-integers-in-a-range-are-covered/

use crate::Solution;

impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        use std::collections::HashSet;
        let all_ranges = ranges
            .into_iter()
            .flat_map(|r| r[0]..=*r.last().unwrap())
            .collect::<HashSet<_>>();
        (left..=right).all(|x| all_ranges.contains(&x))
    }

    pub fn is_covered_line_sweep(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let arr_counter = ranges.iter().fold([0; 52], |mut acc, v| {
            acc[v[0] as usize] += 1;
            acc[(v[1] + 1) as usize] -= 1;
            acc
        });

        let mut res = 0;
        for val in 1..=right {
            res += arr_counter[val as usize];
            if val >= left && res == 0 {
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
        assert_eq!(true, Solution::is_covered(vec![vec![1,2],vec![3,4],vec![5,6]], 2, 5));
    }

    #[test]
    fn test_2() {
        assert_eq!(false, Solution::is_covered(vec![vec![1,10],vec![10,20]], 21, 21));
    }

    #[test]
    fn test_3() {
        assert_eq!(true, Solution::is_covered_line_sweep(vec![vec![1,50]], 1, 50));
    }
}
