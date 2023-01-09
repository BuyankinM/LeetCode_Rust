// 228. Summary Ranges
// https://leetcode.com/problems/summary-ranges/

use crate::Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();

        if nums.is_empty() {
            return result;
        }

        let get_str_range = |start: i32, end: i32| -> String {
            match start == end {
                true => format!("{start}"),
                false => format!("{start}->{end}"),
            }
        };

        let mut start = nums[0];
        let mut end = start;

        for &x in &nums[1..] {
            if x > end + 1 {
                result.push(get_str_range(start, end));
                start = x;
            }
            end = x
        }

        result.push(get_str_range(start, end));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec!["0->2".to_string(), "4->5".to_string(), "7".to_string()],
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![
                "0".to_string(),
                "2->4".to_string(),
                "6".to_string(),
                "8->9".to_string()
            ],
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![
                "-2147483648->-2147483647".to_string(),
                "2147483647".to_string()
            ],
            Solution::summary_ranges(vec![-2147483648, -2147483647, 2147483647])
        );
    }
}
