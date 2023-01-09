// 401. Binary Watch
// https://leetcode.com/problems/binary-watch/

use crate::Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let nu = turned_on as u32;
        let mut result = Vec::new();
        for i in 0i32..12 {
            for j in 0i32..60 {
                if (i.count_ones() + j.count_ones()) == nu {
                    result.push(format!("{i}:{j:02}"));
                }
            }
        }
        result
    }

    pub fn read_binary_watch_func(turned_on: i32) -> Vec<String> {
        (0i32..1023)
            .map(|n| (n.count_ones() as i32, n >> 6, n & 0b111111))
            .filter_map(|(count, h, m)| {
                if count == turned_on && h < 12 && m < 60 {
                    Some(format!("{h}:{m:02}"))
                } else {
                    None
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
            Solution::read_binary_watch(1),
            vec![
                "0:01".to_string(),
                "0:02".to_string(),
                "0:04".to_string(),
                "0:08".to_string(),
                "0:16".to_string(),
                "0:32".to_string(),
                "1:00".to_string(),
                "2:00".to_string(),
                "4:00".to_string(),
                "8:00".to_string(),
            ],
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::read_binary_watch(2),
            vec![
                "0:03".to_string(),
                "0:05".to_string(),
                "0:06".to_string(),
                "0:09".to_string(),
                "0:10".to_string(),
                "0:12".to_string(),
                "0:17".to_string(),
                "0:18".to_string(),
                "0:20".to_string(),
                "0:24".to_string(),
                "0:33".to_string(),
                "0:34".to_string(),
                "0:36".to_string(),
                "0:40".to_string(),
                "0:48".to_string(),
                "1:01".to_string(),
                "1:02".to_string(),
                "1:04".to_string(),
                "1:08".to_string(),
                "1:16".to_string(),
                "1:32".to_string(),
                "2:01".to_string(),
                "2:02".to_string(),
                "2:04".to_string(),
                "2:08".to_string(),
                "2:16".to_string(),
                "2:32".to_string(),
                "3:00".to_string(),
                "4:01".to_string(),
                "4:02".to_string(),
                "4:04".to_string(),
                "4:08".to_string(),
                "4:16".to_string(),
                "4:32".to_string(),
                "5:00".to_string(),
                "6:00".to_string(),
                "8:01".to_string(),
                "8:02".to_string(),
                "8:04".to_string(),
                "8:08".to_string(),
                "8:16".to_string(),
                "8:32".to_string(),
                "9:00".to_string(),
                "10:00".to_string(),
            ],
        );
    }
}
