// 967. Numbers With Same Consecutive Differences
// https://leetcode.com/problems/numbers-with-same-consecutive-differences/

use crate::Solution;

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        use std::collections::VecDeque;

        if k == 0 {
            return (1..10)
                .map(|x| x.to_string().repeat(n as usize).parse().unwrap())
                .collect();
        }

        let mut res = Vec::new();
        let mut q: VecDeque<i32> = (1..10).filter(|&x| x + k < 10 || x - k >= 0).collect();
        let mut pair = [0, 0];
        let min_val_k = 10_i32.pow((n - 1) as u32);

        while let Some(x) = q.pop_front() {
            pair[0] = x % 10 + k;
            pair[1] = x % 10 - k;
            for &y in pair.iter().filter(|&y| (0..10).contains(y)) {
                let next = 10 * x + y;
                match next < min_val_k {
                    true => q.push_back(next),
                    false => res.push(next),
                }
            }
        }
        res
    }

    // https://leetcode.com/problems/numbers-with-same-consecutive-differences/discuss/2522676/Rust-or-Recursive-and-Iterative-DFS-or-With-Comments
    pub fn nums_same_consec_diff_dfs(n: i32, k: i32) -> Vec<i32> {
        let mut stack: Vec<_> = (1..=9)
            .map(|first_digit| (n - 1, first_digit, first_digit))
            .collect();
        let mut rez = vec![];

        while let Some((left, curr, last_digit)) = stack.pop() {
            if left == 0 {
                rez.push(curr);
            } else {
                let (lower, higher) = (last_digit - k, last_digit + k);
                if lower >= 0 {
                    stack.push((left - 1, curr * 10 + lower, lower));
                }
                if k != 0 && higher <= 9 {
                    stack.push((left - 1, curr * 10 + higher, higher));
                }
            }
        }

        rez
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![181, 292, 707, 818, 929],
            Solution::nums_same_consec_diff(3, 7)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![12, 10, 23, 21, 34, 32, 45, 43, 56, 54, 67, 65, 78, 76, 89, 87, 98],
            Solution::nums_same_consec_diff(2, 1)
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![11, 22, 33, 44, 55, 66, 77, 88, 99],
            Solution::nums_same_consec_diff(2, 0)
        );
    }
}
