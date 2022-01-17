// 2138. Divide a String Into Groups of Size k
// https://leetcode.com/problems/divide-a-string-into-groups-of-size-k/

use crate::Solution;

impl Solution {
    pub fn divide_string(mut s: String, k: i32, fill: char) -> Vec<String> {
        let l = s.len();
        let ku = k as usize;
        let rem = l % ku;
        let parts = l / ku + (rem > 0) as usize;
        let mut res = Vec::with_capacity(parts);

        (0..ku - rem).for_each(|_| s.push(fill));
        for i in 0..parts {
            res.push(s[i * ku..(i + 1) * ku].to_string());
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
            vec!["abc".to_string(), "def".to_string(), "ghi".to_string()],
            Solution::divide_string("abcdefghi".to_string(), 3, 'x')
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![
                "abc".to_string(),
                "def".to_string(),
                "ghi".to_string(),
                "jxx".to_string()
            ],
            Solution::divide_string("abcdefghij".to_string(), 3, 'x')
        );
    }
}
