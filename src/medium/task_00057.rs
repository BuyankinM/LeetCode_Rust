// 57. Insert Interval
// https://leetcode.com/problems/insert-interval/description/

use crate::Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![new_interval];
        }

        let mut res = vec![];
        let mut cur_vec = Vec::with_capacity(2);
        let (mut start, mut end) = (new_interval.first(), new_interval.last());

        for mut interval in intervals {
            match (start, end) {
                (None, None) => cur_vec = interval,
                (Some(&s), Some(_)) if s > interval[1] => cur_vec = interval,
                (Some(&s), Some(&e)) if e <= interval[1] => {
                    if e < interval[0] {
                        res.push(new_interval.clone());
                    } else {
                        interval[0] = s.min(interval[0]);
                    }
                    start = None;
                    end = None;
                    cur_vec = interval;
                }

                (Some(&s), Some(_)) if s <= interval[1] => {
                    cur_vec.push(s.min(interval[0]));
                    start = None;
                }

                (None, Some(&e)) if e <= interval[1] => {
                    if e < interval[0] {
                        cur_vec.push(e);
                        res.push(cur_vec.clone());
                        cur_vec = interval;
                    } else {
                        cur_vec.push(interval[1]);
                    }
                    end = None;
                }
                _ => (),
            }

            if cur_vec.len() == 2 {
                res.push(cur_vec.clone());
                cur_vec.clear();
            }
        }

        match (start, end) {
            (Some(_), _) => res.push(new_interval),
            (None, Some(&e)) => {
                cur_vec.push(e);
                res.push(cur_vec.clone());
            }
            _ => (),
        }

        res
    }

    // https://leetcode.com/problems/insert-interval/solutions/395709/beautiful-rust-solution-o-n-0-ms/?orderBy=most_votes&languageTags=rust
    pub fn insert_short(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::{max, min};
        let mut less = vec![];
        let mut more = vec![];
        let mut start = new_interval[0];
        let mut end = new_interval[1];

        for curr in intervals {
            if curr[1] < new_interval[0] {
                less.push(curr);
            } else if curr[0] > new_interval[1] {
                more.push(curr);
            } else {
                start = min(curr[0], start);
                end = max(curr[1], end);
            }
        }
        less.push(vec![start, end]);
        less.append(&mut more);
        less
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![vec![1, 5], vec![6, 9]],
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![vec![1, 2], vec![3, 10], vec![12, 16]],
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            )
        );
    }
}
