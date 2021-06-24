// 1758. Minimum Changes To Make Alternating Binary String
// https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string/

use crate::Solution;

impl Solution {
    pub fn min_operations_alt(s: String) -> i32 {
        let s01 = ['0', '1'];
        let (mut i0, mut i1) = (0, 1);
        let (mut oper0, mut oper1) = (0, 0);

        s.chars().for_each(|c| {
            if c != s01[i0] {
                oper0 += 1
            }
            if c != s01[i1] {
                oper1 += 1
            }
            i0 = (i0 + 1) % 2;
            i1 = (i1 + 1) % 2;
        });

        oper0.min(oper1)
    }

    pub fn min_operations_alt_optimal(s: String) -> i32 {
        let s01 = ['0', '1'];
        let mut i = 0;
        let (mut oper0, mut oper1) = (0, 0);

        s.chars().for_each(|c| {
            match c != s01[i] {
                true => oper0 += 1,
                false => oper1 += 1,
            }
            i = (i + 1) % 2;
        });

        oper0.min(oper1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::min_operations_alt("0100".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::min_operations_alt("10".to_owned()));
    }

    #[test]
    fn test_3() {
        assert_eq!(2, Solution::min_operations_alt_optimal("1111".to_owned()));
    }
}
