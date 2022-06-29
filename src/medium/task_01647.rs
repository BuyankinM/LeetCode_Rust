// 1647. Minimum Deletions to Make Character Frequencies Unique
// https://leetcode.com/problems/minimum-deletions-to-make-character-frequencies-unique/

use crate::Solution;

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut counter_freq = [0; 26];
        s.bytes()
            .for_each(|b| counter_freq[(b - b'a') as usize] += 1);
        counter_freq.sort_unstable_by_key(|&x| -x);

        let mut delete_counter = 0;
        let mut max_freq = s.len() as i32;

        for mut freq in counter_freq {
            if freq > max_freq {
                delete_counter += freq - max_freq;
                freq = max_freq;
            }
            max_freq = 0.max(freq - 1);
        }
        delete_counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_deletions("aab".to_string()), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::min_deletions("aaabbbcc".to_string()), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::min_deletions("ceabaacb".to_string()), 2);
    }
}
