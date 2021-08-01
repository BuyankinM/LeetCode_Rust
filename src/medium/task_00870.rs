// 870. Advantage Shuffle
// https://leetcode.com/problems/advantage-shuffle/
use crate::Solution;

impl Solution {
    fn advantage_count(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let l = a.len();
        let mut res = vec![-1; l];

        let mut ai: Vec<(i32, usize)> = a.iter().enumerate().map(|(i, v)| (*v, i)).collect();
        let mut bi: Vec<(i32, usize)> = b.iter().enumerate().map(|(i, v)| (*v, i)).collect();
        ai.sort_unstable();
        bi.sort_unstable();

        if ai[l - 1].0 <= bi[0].0 {
            return a;
        }

        let mut idx_a = 0;

        for &(val_b, idx_b) in &bi {
            while (idx_a < l && val_b >= ai[idx_a % l].0) || (idx_a >= l && ai[idx_a % l].0 == -1) {
                idx_a += 1
            }

            res[idx_b] = a[ai[idx_a % l].1];
            ai[idx_a % l].0 = -1;
            idx_a += 1;
        }

        res
    }
}
