// 225. Implement Stack using Queues
// https://leetcode.com/problems/implement-stack-using-queues/

use crate::Solution;
use std::collections::VecDeque;
use std::mem;

#[derive(Default)]
struct MyStack {
    q1: VecDeque<i32>,
    q2: VecDeque<i32>,
    top: i32,
}

impl MyStack {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, x: i32) {
        self.q1.push_back(x);
        self.top = x;
    }

    fn pop(&mut self) -> i32 {
        while self.q1.len() > 1 {
            self.top = self.q1.pop_front().unwrap();
            self.q2.push_back(self.top);
        }
        std::mem::swap(&mut self.q1, &mut self.q2);
        self.q2.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        self.top
    }

    fn empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}
