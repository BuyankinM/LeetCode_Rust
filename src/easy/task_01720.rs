// 1720. Decode XORed Array
// https://leetcode.com/problems/decode-xored-array/

use crate::Solution;

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        std::iter::once(&first)
            .chain(encoded.iter())
            .scan(0, |xored, &x| {
                *xored ^= x;
                Some(*xored)
            })
            .collect::<Vec<i32>>()
    }

    pub fn decode_once(mut encoded: Vec<i32>, first: i32) -> Vec<i32> {
        encoded[0] ^= first;
        (1..encoded.len()).for_each(|i| encoded[i] ^= encoded[i - 1]);
        std::iter::once(first).chain(encoded).collect()
    }

    pub fn decode_cycle(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut result = vec![first];
        for i in 0..encoded.len() {
            result.push(result[i] ^ encoded[i]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![1, 0, 2, 1], Solution::decode(vec![1, 2, 3], 1));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![4, 2, 0, 7, 4],
            Solution::decode_once(vec![6, 2, 7, 3], 4)
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![17, 16, 18, 17, 21, 16, 22, 17, 25, 16, 27, 22, 25],
            Solution::decode_cycle(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 13, 15], 17)
        );
    }
}
