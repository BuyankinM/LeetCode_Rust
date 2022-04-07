// 344. Reverse String
// https://leetcode.com/problems/reverse-string/

use crate::Solution;

impl Solution {
    pub fn reverse_string_swap(s: &mut Vec<char>) {
        let l = s.len();
        (0..l / 2).for_each(|idx| s.swap(idx, l - idx - 1));
    }

    pub fn reverse_string_1line(s: &mut Vec<char>) {
        s.reverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string_swap(&mut s);
        assert_eq!(vec!['o', 'l', 'l', 'e', 'h'], s);
    }

    #[test]
    fn test_2() {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        Solution::reverse_string_swap(&mut s);
        assert_eq!(vec!['h', 'a', 'n', 'n', 'a', 'H'], s);
    }
}
