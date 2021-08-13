// 13. Roman to Integer
// https://leetcode.com/problems/roman-to-integer/

use crate::Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;

        let mut roman_map: HashMap<u8, i32> = HashMap::with_capacity(7);
        roman_map.insert(b'I', 1);
        roman_map.insert(b'V', 5);
        roman_map.insert(b'X', 10);
        roman_map.insert(b'L', 50);
        roman_map.insert(b'C', 100);
        roman_map.insert(b'D', 500);
        roman_map.insert(b'M', 1000);

        let (mut res, mut prev) = (0, b' ');

        s.as_bytes().iter().for_each(|&c| match (prev, c) {
            (b'I', b'V')
            | (b'I', b'X')
            | (b'X', b'L')
            | (b'X', b'C')
            | (b'C', b'D')
            | (b'C', b'M') => {
                res += roman_map[&c] - 2 * roman_map[&prev]; // correction with prev symbol IV = (+1+5-2) = 4
                prev = b' '
            }

            (_, b) if (b == b'I' || b == b'X' || b == b'C') => {
                res += roman_map[&b];
                prev = b
            }

            _ => {
                res += roman_map[&c];
                prev = b' '
            }
        });

        res
    }

    pub fn roman_to_int_optimal(s: String) -> i32 {
        let (mut res, mut last) = (0, 0);
        for c in s.chars().rev() {
            let curr = match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };

            res += match curr >= last {
                true => curr,
                false => -curr,
            };

            last = curr;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::roman_to_int("III".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, Solution::roman_to_int("IV".to_owned()));
    }

    #[test]
    fn test_3() {
        assert_eq!(9, Solution::roman_to_int("IX".to_owned()));
    }

    #[test]
    fn test_4() {
        assert_eq!(58, Solution::roman_to_int("LVIII".to_owned()));
    }

    #[test]
    fn test_5() {
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_owned()));
    }

    #[test]
    fn test_6() {
        assert_eq!(621, Solution::roman_to_int("DCXXI".to_owned()));
    }

    #[test]
    fn test_7() {
        assert_eq!(621, Solution::roman_to_int_optimal("DCXXI".to_owned()));
    }
}
