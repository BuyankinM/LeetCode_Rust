// 1461. Check If a String Contains All Binary Codes of Size K
// https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/

use crate::Solution;
use std::str::from_utf8;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let kk = k as usize;
        let l: usize = 2_usize.pow(kk as u32);

        if s.len() < (l + (k as usize) - 1) {
            return false;
        }

        let mut a: Vec<u8> = vec![0; l];
        let mut n: usize = 0;
        let sb = &s.as_bytes();

        for i in 0..=(s.len() - kk) {
            let ss = from_utf8(&sb[i..i + kk]).unwrap();
            let ind: usize = usize::from_str_radix(ss, 2).unwrap();
            if a[ind] == 0 {
                a[ind] = 1;
                n += 1;
            }
            if n == l {
                return true;
            }
        }

        false
    }

    pub fn has_all_codes_best(s: String, k: i32) -> bool {
        let k = k as usize;
        let mut v = vec![false; 1 << k];
        let (mut n, mut count) = (0, 1 << k);
        let mask = (1 << k) - 1;
        for (i, &b) in s.as_bytes().iter().enumerate() {
            n = ((n << 1) & mask) + if b == b'1' { 1 } else { 0 };
            if i + 1 < k || v[n] {
                continue;
            }
            v[n] = true;
            count -= 1;
            if count == 0 {
                return true;
            }
        }
        false
    }
}
