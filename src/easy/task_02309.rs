// 2309. Greatest English Letter in Upper and Lower Case
// https://leetcode.com/problems/greatest-english-letter-in-upper-and-lower-case/

use crate::Solution;

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut counter = [false; 58];
        s.bytes().for_each(|b| counter[(b - b'A') as usize] = true);

        match counter[..26]
            .iter()
            .enumerate()
            .filter(|&(i, &val)| val && counter[i + 32])
            .max()
        {
            Some((i, _)) => ((i as u8 + b'A') as char).to_string(),
            None => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "E".to_string(),
            Solution::greatest_letter("lEeTcOdE".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "R".to_string(),
            Solution::greatest_letter("arRAzFif".to_string())
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            "".to_string(),
            Solution::greatest_letter("AbCdEfGhIjK".to_string())
        );
    }
}
