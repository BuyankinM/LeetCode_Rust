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

    // https://leetcode.com/problems/split-array-into-consecutive-subsequences/discuss/2448900/Rust-or-100-Faster-or-100-Less-Memory-or-BinaryHeap-Reverse
    pub fn is_possible_heap(nums: Vec<i32>) -> bool {
        use std::cmp::Reverse;

        let mut heap = std::collections::BinaryHeap::new();

        for n in nums {
            while let Some(Reverse((e, c))) = heap.pop() {
                if e + 1 == n {
                    heap.push(Reverse((n, c + 1)));
                    break;
                } else if e == n {
                    heap.push(Reverse((e, c)));
                    heap.push(Reverse((n, 1)));
                    break;
                } else if c < 3 {
                    return false;
                }
            }

            if heap.is_empty() {
                heap.push(Reverse((n, 1)));
            }
        }
        heap.iter().all(|Reverse((_, c))| *c >= 3)
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

    #[test]
    fn test_4() {
        assert!(Solution::is_possible_heap(vec![1, 2, 3, 3, 4, 4, 5, 5]));
    }
}
