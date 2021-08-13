// 1370. Increasing Decreasing String
// https://leetcode.com/problems/increasing-decreasing-string/

use crate::Solution;

impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut res = String::with_capacity(s.len());
        let mut counter = [0; 26];
        let (mut incr, mut ind) = (1, 0);

        s.as_bytes()
            .iter()
            .for_each(|b| counter[(*b - b'a') as usize] += 1);

        while res.len() < s.len() {
            if counter[ind as usize] > 0 {
                res.push((b'a' + ind as u8) as char);
                counter[ind as usize] -= 1;
            };

            ind += incr;
            if !(0..=25).contains(&ind) {
                ind = if ind < 0 { 0 } else { 25 };
                incr *= -1;
            };
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "abccbaabccba".to_owned(),
            Solution::sort_string("aaaabbbbcccc".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!("art".to_owned(), Solution::sort_string("rat".to_owned()));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "cdelotee".to_owned(),
            Solution::sort_string("leetcode".to_owned())
        );
    }
}
