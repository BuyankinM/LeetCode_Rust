// 1925. Count Square Sum Triples
// https://leetcode.com/problems/count-square-sum-triples/

use crate::Solution;

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        use std::collections::HashSet;

        if n <= 2 {
            return 0;
        }

        let (l, max_sum) = (n as usize, n * n);
        let mut count = 0;
        let squares_vec = (1..=n).map(|x| x * x).collect::<Vec<_>>();
        let squares_set = squares_vec.iter().collect::<HashSet<_>>();

        for (i, &square_1) in squares_vec[..(l - 2)].iter().enumerate() {
            for &square_2 in &squares_vec[i + 1..=(l - 1)] {
                match square_1 + square_2 {
                    sum if sum <= max_sum => count += 2 * (squares_set.contains(&sum) as i32),
                    _ => break, // early stop
                };
            }
        }
        count
    }

    pub fn count_triples_vec(n: i32) -> i32 {
        if n <= 2 {
            return 0;
        }

        let (l, max_sum) = (n as usize, (n * n) as usize);
        let mut res = 0;

        let mut squares = Vec::with_capacity(l);
        let mut squares_check = vec![0; max_sum + 1];
        (1..=l).map(|x| x * x).for_each(|x| {
            squares_check[x] = 1;
            squares.push(x)
        });

        for (i, &a) in squares[..l - 2].iter().enumerate() {
            for &b in &squares[i + 1..l - 1] {
                match a + b {
                    sum if sum <= max_sum => res += squares_check[sum] * 2,
                    _ => break, // early stop
                };
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::count_triples(5));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, Solution::count_triples(10));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::count_triples(1));
    }
    
    #[test]
    fn test_4() {
        assert_eq!(330, Solution::count_triples(250));
    }

    #[test]
    fn test_5() {
        assert_eq!(330, Solution::count_triples_vec(250));
    }
}
