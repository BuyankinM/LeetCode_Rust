// 155. Min Stack
// https://leetcode.com/problems/min-stack/

#[derive(Default)]
struct MinStack {
    data: Vec<(i32, i32)>,
}

use crate::Solution;

impl MinStack {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, val: i32) {
        if self.data.is_empty() {
            self.data.push((val, val));
        } else {
            let min_val = self.data.last().unwrap().1.min(val);
            self.data.push((val, min_val));
        }
    }

    fn pop(&mut self) {
        self.data.pop();
    }

    fn top(&self) -> i32 {
        self.data.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.data.last().unwrap().1
    }
}
