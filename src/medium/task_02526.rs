// 2526. Find Consecutive Integers from a Data Stream
// https://leetcode.com/problems/find-consecutive-integers-from-a-data-stream/

use crate::Solution;

struct DataStream {
    value: i32,
    k: i32,
    n: i32,
}

impl DataStream {
    fn new(value: i32, k: i32) -> Self {
        Self { value, k, n: 0 }
    }

    fn consec(&mut self, num: i32) -> bool {
        match num == self.value {
            true => self.n += 1,
            false => self.n = 0,
        }
        self.n >= self.k
    }
}
