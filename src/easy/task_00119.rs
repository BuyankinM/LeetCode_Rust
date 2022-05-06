// 119. Pascal's Triangle II
// https://leetcode.com/problems/pascals-triangle-ii/

use crate::Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row = vec![1];
        for _ in 0..row_index {
            let mut next_row = vec![1];
            row.iter()
                .zip(row[1..].iter())
                .for_each(|(a, b)| next_row.push(*a + *b));
            next_row.push(1);
            row = next_row;
        }
        row
    }

    // https://leetcode.com/problems/pascals-triangle-ii/discuss/629622/Math-formula-based-Rust-solution
    pub fn get_row_math(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        let mut res = vec![1];

        for i in 1..row_index + 1 {
            let tmp = res[i - 1] as usize * (row_index + 1 - i) / i;
            res.push(tmp as i32);
        }

        res
    }

    // https://leetcode.com/problems/pascals-triangle-ii/discuss/1840543/rust-dp-without-extra-space!
    pub fn get_row_dp(row_index: i32) -> Vec<i32> {
        // 1 0 0 0
        // 1 1 0 0
        // 1 2 1 0
        // 1 3 3 1
        // f(x, y) = f(x - 1, y - 1) + f(x, y - 1)

        let row_index = row_index as usize;
        let mut arr = vec![0; row_index + 1];
        arr[0] = 1;

        for i in 1..arr.len() {
            for j in (1..=i).rev() {
                arr[j] += arr[j - 1];
            }
        }

        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::get_row_dp(3), vec![1, 3, 3, 1]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::get_row(0), vec![1]);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::get_row(9),
            vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1]
        );
    }
}
