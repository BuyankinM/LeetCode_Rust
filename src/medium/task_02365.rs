// 2365. Task Scheduler II
// https://leetcode.com/problems/task-scheduler-ii/

use crate::Solution;

impl Solution {
    pub fn task_scheduler_ii(tasks: Vec<i32>, space: i32) -> i64 {
        let s = space as i64;
        let (mut res, mut i) = (0, 0);
        let mut hm = std::collections::HashMap::with_capacity(tasks.len());

        for &x in &tasks {
            let e = hm.entry(x).or_insert(-100_001);
            let step = 1.max(s - (i - *e) + 1);
            i += step;
            res += step;
            *e = i;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::task_scheduler_ii(vec![1, 2, 1, 2, 3, 1], 3), 9);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::task_scheduler_ii(vec![5, 8, 8, 5], 2), 6);
    }
}
