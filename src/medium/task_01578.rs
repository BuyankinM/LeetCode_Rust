// 1578. Minimum Time to Make Rope Colorful
// https://leetcode.com/problems/minimum-time-to-make-rope-colorful/

use crate::Solution;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let (mut total_time, mut max_time) = (0, 0);
        let mut prev_color = b' ';

        for (color, &cur_time) in colors.bytes().zip(needed_time.iter()) {
            if prev_color != color {
                max_time = 0;
            }
            total_time += cur_time.min(max_time);
            max_time = cur_time.max(max_time);
            prev_color = color;
        }

        total_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::min_cost("abaac".to_string(), vec![1, 2, 3, 4, 5]),
            3
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::min_cost("abc".to_string(), vec![1, 2, 3]), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::min_cost("aabaa".to_string(), vec![1, 2, 3, 4, 1]),
            2
        );
    }
}
