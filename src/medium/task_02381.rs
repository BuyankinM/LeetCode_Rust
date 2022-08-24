// 2381. Shifting Letters II
// https://leetcode.com/problems/shifting-letters-ii/

use crate::Solution;

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut dirs = vec![0; s.len() + 1];

        shifts.iter().for_each(|v| {
            let dir = if v[2] == 0 { -1 } else { 1 };
            dirs[v[0] as usize] += dir;
            dirs[(v[1] + 1) as usize] -= dir;
        });

        let mut delta = 0;

        s.chars()
            .zip(dirs.iter())
            .map(|(c, &d)| {
                delta += d;
                let new_c = match ((c as i32 - 97) + delta) % 26 {
                    v if v < 0 => v + 26,
                    v => v,
                };
                (new_c as u8 + b'a') as char
            })
            .collect()
    }
}
