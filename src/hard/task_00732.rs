// 732. My Calendar III
// https://leetcode.com/problems/my-calendar-iii/

use crate::Solution;

use std::collections::BTreeMap;

#[derive(Default)]
struct MyCalendarThree {
    tree: BTreeMap<i32, i32>,
}

impl MyCalendarThree {
    fn new() -> Self {
        Default::default()
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        self.tree.entry(start).and_modify(|x| *x += 1).or_insert(1);
        self.tree.entry(end).and_modify(|x| *x -= 1).or_insert(-1);
        self.tree
            .values()
            .fold((0, 0), |(cur, res), &x| (cur + x, res.max(cur + x)))
            .1
    }
}
