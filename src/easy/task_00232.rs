// 232. Implement Queue using Stacks
// https://leetcode.com/problems/implement-queue-using-stacks/

use crate::Solution;

#[derive(Default)]
struct MyQueue {
    stack_back: Vec<i32>,
    stack_front: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, x: i32) {
        self.stack_back.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.move_back_to_front();
        self.stack_front.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        self.move_back_to_front();
        *self.stack_front.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack_front.is_empty() && self.stack_back.is_empty()
    }

    fn move_back_to_front(&mut self) {
        if self.stack_front.is_empty() {
            while let Some(x) = self.stack_back.pop() {
                self.stack_front.push(x);
            }
        }
    }
}
