// 2531. Make Number of Distinct Characters Equal
// https://leetcode.com/problems/make-number-of-distinct-characters-equal/

use crate::Solution;

impl Solution {
    pub fn is_it_possible(word1: String, word2: String) -> bool {
        let counter = |s: &str| {
            s.bytes().fold([0; 26], |mut acc, b| {
                acc[(b - b'a') as usize] += 1;
                acc
            })
        };

        let idx = |arr: &[i32; 26]| -> Vec<usize> {
            arr.iter()
                .enumerate()
                .filter_map(|(i, n)| (*n > 0).then_some(i))
                .collect()
        };

        let move_letters = |c1: &mut [i32; 26], c2: &mut [i32; 26], i: usize, j: usize, k: i32| {
            c1[i] -= k;
            c2[i] += k;
            c1[j] += k;
            c2[j] -= k;
        };

        let check_distinct = |c1: &[i32; 26], c2: &[i32; 26]| {
            c1.iter().filter(|n| **n > 0).count() == c2.iter().filter(|n| **n > 0).count()
        };

        let mut c1 = counter(word1.as_str());
        let mut c2 = counter(word2.as_str());

        for &i in &idx(&c1) {
            for &j in &idx(&c2) {
                move_letters(&mut c1, &mut c2, i, j, 1);
                if check_distinct(&c1, &c2) {
                    return true;
                }
                move_letters(&mut c1, &mut c2, i, j, -1);
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(!Solution::is_it_possible("ac".to_string(), "b".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(Solution::is_it_possible(
            "abcc".to_string(),
            "aab".to_string()
        ));
    }

    #[test]
    fn test_3() {
        assert!(Solution::is_it_possible(
            "abcde".to_string(),
            "fghij".to_string()
        ));
    }

    #[test]
    fn test_4() {
        assert!(!Solution::is_it_possible("a".to_string(), "bb".to_string()));
    }
}
