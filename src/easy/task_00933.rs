// 933. Number of Recent Calls
// https://leetcode.com/problems/number-of-recent-calls/

use crate::Solution;
use std::collections::VecDeque;

#[derive(Default)]
struct RecentCounter {
    last_pings: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        Default::default()
    }

    fn ping(&mut self, t: i32) -> i32 {
        let low = t - 3000;
        self.last_pings.push_back(t);

        while let Some(first) = self.last_pings.front() {
            match *first < low {
                true => self.last_pings.pop_front(),
                false => break,
            };
        }

        self.last_pings.len() as _
    }
}
