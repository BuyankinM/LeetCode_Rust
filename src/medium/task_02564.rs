// 2564. Substring XOR Queries
// https://leetcode.com/problems/substring-xor-queries/

use crate::Solution;

const MAX_LENGTH: usize = 30;

impl Solution {
    pub fn substring_xor_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut hm = std::collections::HashMap::new();
        let mut zero_found = false;
        let bytes = s.as_bytes();

        for (start, b) in bytes.iter().map(|&b| (b - b'0') as i32).enumerate() {
            if b == 0 {
                if !zero_found {
                    hm.insert(0, (start, start));
                    zero_found = true;
                }
            } else {
                let mut num = 0;
                let end = (start + MAX_LENGTH).min(bytes.len());
                bytes[start..end].iter().enumerate().for_each(|(i, &b)| {
                    num = (num << 1) + (b - b'0') as i32;
                    hm.entry(num).or_insert((start, start + i));
                })
            }
        }

        queries
            .iter()
            .map(|q| {
                let val = q[0] ^ q[1];
                match hm.get(&val) {
                    Some(&(start, end)) => vec![start as i32, end as i32],
                    None => vec![-1, -1],
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![vec![0, 2], vec![2, 3]],
            Solution::substring_xor_queries("101101".to_string(), vec![vec![0, 5], vec![1, 2]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![vec![-1, -1]],
            Solution::substring_xor_queries("0101".to_string(), vec![vec![12, 8]])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![vec![0, 0]],
            Solution::substring_xor_queries("1".to_string(), vec![vec![4, 5]])
        );
    }
}
