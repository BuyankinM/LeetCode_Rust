// 2336. Smallest Number in Infinite Set
// https://leetcode.com/problems/smallest-number-in-infinite-set/

use crate::Solution;

use std::collections::HashSet;

#[derive(Debug, Default)]
struct SmallestInfiniteSet {
    min_val: i32,
    removed: HashSet<i32>,
}

impl SmallestInfiniteSet {
    fn new() -> Self {
        SmallestInfiniteSet {
            min_val: 1,
            removed: HashSet::new(),
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        loop {
            if !self.removed.contains(&self.min_val) {
                self.removed.insert(self.min_val);
                break self.min_val;
            }
            self.min_val += 1;
        }
    }

    fn add_back(&mut self, num: i32) {
        if num < self.min_val {
            self.min_val = num;
        }
        self.removed.remove(&num);
    }
}
