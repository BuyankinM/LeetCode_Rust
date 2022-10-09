// 2432. The Employee That Worked on the Longest Task
// https://leetcode.com/problems/the-employee-that-worked-on-the-longest-task/

use crate::Solution;

impl Solution {
    pub fn hardest_worker(_n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let (mut max_task, mut prev_time, mut res_id) = (0, 0, 0);
        logs.iter().for_each(|log| {
            if let &[id, time] = log.as_slice() {
                let cur_len = time - prev_time;
                if cur_len > max_task || cur_len == max_task && id < res_id {
                    max_task = cur_len;
                    res_id = id;
                }
                prev_time = time;
            }
        });
        res_id
    }

    // https://leetcode.com/problems/the-employee-that-worked-on-the-longest-task/discuss/2679110/Rust-or-Max-Min-or-With-Comments
    pub fn hardest_worker_scan(_n: i32, logs: Vec<Vec<i32>>) -> i32 {
        logs.into_iter()
            .scan(0, |t, log| {
                let rez = Some((log[0], log[1] - *t));
                *t = log[1];
                rez
            })
            .max_by(|w1, w2| w1.1.cmp(&w2.1).then(w2.0.cmp(&w1.0)))
            .unwrap()
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::hardest_worker(10, vec![vec![0, 3], vec![2, 5], vec![0, 9], vec![1, 15]]),
            1
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::hardest_worker(26, vec![vec![1, 1], vec![3, 7], vec![2, 12], vec![7, 17]]),
            3
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::hardest_worker(2, vec![vec![0, 10], vec![1, 20]]),
            0
        );
    }
}
