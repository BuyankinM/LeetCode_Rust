// 278. First Bad Version
// https://leetcode.com/problems/first-bad-version/

use crate::Solution;

impl Solution {
    fn isBadVersion(&self, version: i32) -> bool {
        // stub function
        version != 0
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut low, mut high) = (1, n);
        while low < high {
            let mid = low + (high - low) / 2;
            match self.isBadVersion(mid) {
                true => high = mid,
                false => low = mid + 1,
            }
        }
        low
    }
}
