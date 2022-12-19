// 38. Count and Say
// https://leetcode.com/problems/count-and-say/

use crate::Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        fn get_pairs(s: String) -> Vec<(char, i32)> {
            let mut v = Vec::new();
            let (mut prev_dig, mut num_dig) = (' ', 0);
            for d in s.chars() {
                if d != prev_dig && prev_dig != ' ' {
                    v.push((prev_dig, num_dig));
                    num_dig = 0;
                }
                prev_dig = d;
                num_dig += 1;
            }
            v.push((prev_dig, num_dig));
            v
        }

        fn get_num_from_pairs(pairs: Vec<(char, i32)>) -> String {
            pairs
                .iter()
                .fold(String::new(), |s, &(dig, count)| format!("{s}{count}{dig}"))
        }

        let mut res = "1".to_string();
        for _ in 1..n {
            res = get_num_from_pairs(get_pairs(res));
        }
        res
    }

    // https://leetcode.com/problems/count-and-say/discuss/2717340/rust-four-solutions-with-comments
    pub fn count_and_say_optimal(n: i32) -> String {
        use std::iter::once;

        let mut curr = vec![1];
        for _ in 1..n {
            let mut next = vec![];
            let mut slow = 0;
            for fast in 0..=curr.len() {
                if fast == curr.len() || curr[slow] != curr[fast] {
                    next.extend(once((fast - slow) as u8).chain(once(curr[slow])));
                    slow = fast;
                }
            }
            curr = next;
        }
        curr.into_iter()
            .map(|digit| (digit + b'0') as char)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("1211".to_string(), Solution::count_and_say(4));
    }

    #[test]
    fn test_2() {
        assert_eq!("1".to_string(), Solution::count_and_say(1));
    }
}
