// 1652. Defuse the Bomb
// https://leetcode.com/problems/defuse-the-bomb/

use crate::Solution;

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let l = code.len();

        if k == 0 {
            return vec![0; l];
        };

        let mut res = Vec::with_capacity(l);
        let mut start = match k.signum() {
            -1 => l - k.abs() as usize,
            1 => 1,
            _ => 0,
        };

        for _ in 0..l {
            let mut s = 0;
            for j in start..(start + k.abs() as usize) {
                s += code[j % l];
            }
            res.push(s);
            start += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![12, 10, 16, 13], Solution::decrypt(vec![5, 7, 1, 4], 3));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![0, 0, 0, 0], Solution::decrypt(vec![0, 0, 0, 0], 0));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![12, 5, 6, 13], Solution::decrypt(vec![2, 4, 9, 3], -2));
    }
}
