// 703. Kth Largest Element in a Stream
// https://leetcode.com/problems/kth-largest-element-in-a-stream/

use crate::Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    size: usize,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut largest = KthLargest {
            heap: BinaryHeap::with_capacity(k as usize),
            size: k as usize,
        };
        nums.iter().for_each(|&x| largest.reorder_max(x));
        largest
    }

    fn add(&mut self, val: i32) -> i32 {
        self.reorder_max(val);
        if let Some(Reverse(x)) = self.heap.peek() {
            return *x;
        }
        unreachable!();
    }

    fn reorder_max(&mut self, x: i32) {
        match self.heap.peek() {
            Some(Reverse(min)) => {
                if x > *min || self.heap.len() < self.size {
                    self.heap.push(Reverse(x));
                    if self.heap.len() > self.size {
                        self.heap.pop();
                    }
                }
            }
            _ => self.heap.push(Reverse(x)),
        }
    }
}
