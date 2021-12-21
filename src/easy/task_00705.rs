// 705. Design HashSet
// https://leetcode.com/problems/design-hashset/

use crate::Solution;

struct MyHashSet {
    values: [bool; 1_000_001],
}

impl MyHashSet {
    fn new() -> Self {
        MyHashSet {
            values: [false; 1_000_001],
        }
    }

    fn add(&mut self, key: i32) {
        self.values[key as usize] = true;
    }

    fn remove(&mut self, key: i32) {
        self.values[key as usize] = false;
    }

    fn contains(&self, key: i32) -> bool {
        self.values[key as usize]
    }
}
