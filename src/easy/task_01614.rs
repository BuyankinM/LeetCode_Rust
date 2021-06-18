// 1614. Maximum Nesting Depth of the Parentheses
// https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/

use crate::Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let (mut cur_depth, mut max_depth) = (0, 0);
        s.chars().for_each(|x| match x {
            '(' => {
                cur_depth += 1;
                max_depth = max_depth.max(cur_depth)
            }
            ')' => cur_depth -= 1,
            _ => (),
        });

        max_depth
    }

    pub fn max_depth_fold(s: String) -> i32 {
        let mut mdepth: i32 = 0;
        s.chars().fold(0, |depth, c| match c {
            '(' => depth + 1,
            ')' => {
                if depth > mdepth {
                    mdepth = depth;
                }
                depth - 1
            }
            _ => depth,
        });
        mdepth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::max_depth("(1+(2*3)+((8)/4))+1".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::max_depth("(1)+((2))+(((3)))".to_owned()));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::max_depth_fold("1+(2*3)/(2-1)".to_owned()));
    }

    #[test]
    fn test_4() {
        assert_eq!(0, Solution::max_depth_fold("1".to_owned()));
    }
}
