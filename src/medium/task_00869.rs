// 869. Reordered Power of 2
// https://leetcode.com/problems/reordered-power-of-2/
use crate::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        fn get_hash_bytes_power2() -> HashSet<Vec<u8>> {
            let mut a = 1u64;
            let mut h = HashSet::new();
            for _ in 0..32 {
                let mut b = a.to_string().clone().into_bytes();
                b.sort_unstable();
                h.insert(b);
                a <<= 1;
            }
            h
        }

        let h = get_hash_bytes_power2();
        let mut nb = n.to_string().clone().into_bytes();
        nb.sort_unstable();

        h.contains(&nb)
    }
}
