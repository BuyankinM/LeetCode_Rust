// 389. Find the Difference
// https://leetcode.com/problems/find-the-difference/

use crate::Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut counter = [0; 26];
        let index = |b: u8| -> usize { (b - b'a') as _ };
        s.as_bytes()
            .iter()
            .map(|&b| (index(b), 1))
            .chain(t.as_bytes().iter().map(|&b| (index(b), -1)))
            .for_each(|(ind, x)| counter[ind] += x);

        (counter.iter().position(|x| *x == -1).unwrap() as u8 + b'a') as char
    }

    // https://leetcode.com/problems/find-the-difference/discuss/729937/Rust-extended-to-Unicode
    pub fn find_the_difference_xor(s: String, t: String) -> char {
        let mut ans = 0u32;
        for ch in s.chars() {
            ans ^= ch as u32;
        }
        for ch in t.chars() {
            ans ^= ch as u32;
        }
        std::char::from_u32(ans).unwrap()
    }

    // https://leetcode.com/problems/find-the-difference/discuss/861928/Rust-1-line-solution
    pub fn find_the_difference_xor_oneliner(s: String, t: String) -> char {
        s.as_bytes()
            .iter()
            .chain(t.as_bytes())
            .fold(0, |acc, &x| acc ^ x) as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            'e',
            Solution::find_the_difference("abcd".to_string(), "abcde".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            'y',
            Solution::find_the_difference("".to_string(), "y".to_string())
        );
    }
}
