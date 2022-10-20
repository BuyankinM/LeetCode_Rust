// 2438. Range Product Queries of Powers
// https://leetcode.com/problems/range-product-queries-of-powers/

use crate::Solution;

impl Solution {
    pub fn product_queries(mut n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut powers = Vec::new();
        let mut p = 0;

        while n > 0 {
            if n & 1 == 1 {
                powers.push(p);
            }
            p += 1;
            n >>= 1;
        }

        queries
            .iter()
            .map(|pair| {
                powers[pair[0] as usize..=pair[1] as usize]
                    .iter()
                    .fold(1_i64, |s, p| (s << p) % 1_000_000_007) as i32
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
            Solution::product_queries(15, vec![vec![0, 1], vec![2, 2], vec![0, 3]]),
            vec![2, 4, 64]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::product_queries(2, vec![vec![0, 0]]), vec![2]);
    }
}
