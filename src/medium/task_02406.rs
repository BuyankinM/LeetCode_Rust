// 2406. Divide Intervals Into Minimum Number of Groups
// https://leetcode.com/problems/divide-intervals-into-minimum-number-of-groups/

use crate::Solution;

impl Solution {
    pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        let mut acc = vec![0; 1_000_002];
        intervals.iter().for_each(|pair| {
            acc[pair[0] as usize] += 1;
            acc[(pair[1] + 1) as usize] -= 1;
        });

        let (mut cur, mut res) = (0, 0);
        acc.iter().for_each(|&x| {
            cur += x;
            res = res.max(cur)
        });

        res
    }

    // https://leetcode.com/problems/divide-intervals-into-minimum-number-of-groups/discuss/2563560/Rust-55ms-fastest-solution-(100)-with-BinaryHeap-(with-detailed-comments)
    pub fn min_groups_heap_1(mut intervals: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        // [1] initialize storage for last elements in each group
        let mut groups_last: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(intervals.len());

        // [2] sorting of intervals allows us to check the intersection of each interval
        //     with only the last interval in each group
        intervals.sort_unstable_by_key(|interval| interval[0]);

        // [3] iterate over sorted intervals
        intervals.into_iter().for_each(|interval| {
            // [4] searching for a group to attach new 'interval',
            //     i.e., taking a group with 'end' < 'start of interval';
            //     it is sufficient to check the group with the minimal 'end' value,
            //     thus, we use BinaryHeap for quick (log-complexity) access
            if let Some(Reverse(end)) = groups_last.peek() {
                // [5] if there is such a group, prepare to update it...
                if *end < interval[0] {
                    groups_last.pop();
                }
            }
            // [6] either update an existing group or add a new one
            groups_last.push(Reverse(interval[1]));
        });

        groups_last.len() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::min_groups(vec![
                vec![5, 10],
                vec![6, 8],
                vec![1, 5],
                vec![2, 3],
                vec![1, 10]
            ]),
            3
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::min_groups(vec![vec![1, 3], vec![5, 6], vec![8, 10], vec![11, 13]]),
            1
        );
    }
}
