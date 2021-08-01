// 1374. Generate a String With Characters That Have Odd Counts
// https://leetcode.com/problems/generate-a-string-with-characters-that-have-odd-counts/

use crate::Solution;

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        use std::iter;

        iter::repeat('a')
            .take((n - 1 + n % 2) as usize)
            .chain(iter::once('b').take((1 - n % 2) as usize))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("aaab".to_owned(), Solution::generate_the_string(4));
    }

    #[test]
    fn test_2() {
        assert_eq!("aaa".to_owned(), Solution::generate_the_string(3));
    }
}
