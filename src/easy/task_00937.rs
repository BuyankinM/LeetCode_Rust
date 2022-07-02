// 937. Reorder Data in Log Files
// https://leetcode.com/problems/reorder-data-in-log-files/

use crate::Solution;

pub struct DataSort<'a> {
    pub flag_let_dig: usize,
    pub id: &'a str,
    pub content: &'a str,
    pub s: &'a String,
}

impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut logs_sort = logs
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let (id, content) = s.split_once(' ').unwrap();
                let flag_let_dig = match content.chars().next() {
                    Some(c) if c.is_alphabetic() => 0,
                    _ => i + 1,
                };

                DataSort {
                    flag_let_dig,
                    content,
                    id,
                    s,
                }
            })
            .collect::<Vec<_>>();
        logs_sort.sort_unstable_by(|a, b| {
            a.flag_let_dig
                .cmp(&b.flag_let_dig)
                .then(a.content.cmp(b.content))
                .then(a.id.cmp(b.id))
        });
        logs_sort.into_iter().map(|ks| ks.s.clone()).collect()
    }

    // https://leetcode.com/problems/reorder-data-in-log-files/discuss/868563/Rust-solution
    pub fn reorder_log_files_short(mut logs: Vec<String>) -> Vec<String> {
        use std::cmp::Ordering;

        fn is_digit_log(rest: &str) -> bool {
            rest.as_bytes()[0].is_ascii_digit()
        }

        logs.sort_by(|log1, log2| -> Ordering {
            // Split the two logs to two part: identifier and the rest respectively.
            let log1_id_rest = log1.splitn(2, ' ').collect::<Vec<&str>>();
            let log2_id_rest = log2.splitn(2, ' ').collect::<Vec<&str>>();

            match (is_digit_log(log1_id_rest[1]), is_digit_log(log2_id_rest[1])) {
                (true, true) => Ordering::Equal,
                (false, false) => {
                    (log1_id_rest[1], log1_id_rest[0]).cmp(&(log2_id_rest[1], log2_id_rest[0]))
                }
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
            }
        });

        logs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![
                "let1 art can".to_string(),
                "let3 art zero".to_string(),
                "let2 own kit dig".to_string(),
                "dig1 8 1 5 1".to_string(),
                "dig2 3 6".to_string()
            ],
            Solution::reorder_log_files(vec![
                "dig1 8 1 5 1".to_string(),
                "let1 art can".to_string(),
                "dig2 3 6".to_string(),
                "let2 own kit dig".to_string(),
                "let3 art zero".to_string()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![
                "g1 act car".to_string(),
                "a8 act zoo".to_string(),
                "ab1 off key dog".to_string(),
                "a1 9 2 3 1".to_string(),
                "zo4 4 7".to_string(),
            ],
            Solution::reorder_log_files(vec![
                "a1 9 2 3 1".to_string(),
                "g1 act car".to_string(),
                "zo4 4 7".to_string(),
                "ab1 off key dog".to_string(),
                "a8 act zoo".to_string()
            ])
        );
    }
}
