// 2335. Minimum Amount of Time to Fill Cups
// https://leetcode.com/problems/minimum-amount-of-time-to-fill-cups/

use crate::Solution;

impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;

        let mut res = 0;
        let mut heap = amount
            .iter()
            .filter(|x| **x > 0)
            .cloned()
            .collect::<BinaryHeap<_>>();

        while !heap.is_empty() {
            match heap.len() >= 2 {
                true => {
                    let (x1, x2) = (heap.pop().unwrap(), heap.pop().unwrap());
                    if x1 > 1 {
                        heap.push(x1 - 1);
                    }
                    if x2 > 1 {
                        heap.push(x2 - 1);
                    }
                }
                false => {
                    res += heap.pop().unwrap();
                    break;
                }
            }
            res += 1;
        }
        res
    }

    // https://leetcode.com/problems/minimum-amount-of-time-to-fill-cups/discuss/2265309/Simulation-and-Formula
    pub fn fill_cups_opt(amount: Vec<i32>) -> i32 {
        let (mx, sum) = amount
            .iter()
            .fold((i32::MIN, 0), |(mx, sum), &x| (mx.max(x), sum + x));
        mx.max((sum + 1) / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::fill_cups(vec![1, 4, 2]));
    }

    #[test]
    fn test_2() {
        assert_eq!(7, Solution::fill_cups(vec![5, 4, 4]));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::fill_cups(vec![0, 0, 0]));
    }

    #[test]
    fn test_4() {
        assert_eq!(7, Solution::fill_cups_opt(vec![5, 4, 4]));
    }
}
