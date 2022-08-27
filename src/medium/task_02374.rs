// 2374. Node With Highest Edge Score
// https://leetcode.com/problems/node-with-highest-edge-score/

use crate::Solution;

impl Solution {
    pub fn edge_score(edges: Vec<i32>) -> i32 {
        let mut counter = vec![(0_u8, 0_u32); 100_001];
        let mut max_val = (0, 0);

        for (&x, i) in edges.iter().zip(0..) {
            let elem = &mut counter[x as usize];
            let (over_num, val) = *elem;

            *elem = match val.overflowing_add(i) {
                (new_val, false) => (over_num, new_val),
                (new_val, true) => (over_num + 1, new_val),
            };

            if *elem > max_val {
                max_val = *elem;
            }
        }
        counter.into_iter().position(|val| val == max_val).unwrap() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::edge_score(vec![1, 0, 0, 0, 0, 7, 7, 5]), 7);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::edge_score(vec![2, 0, 0, 2]), 0);
    }
}
