// 12. Integer to Roman
// https://leetcode.com/problems/integer-to-roman/

use crate::Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        fn make_roman_part(res: &mut String, n: usize, d: &str, f: &str, t: &str) {
            match n {
                0 => {}
                1..=3 => *res += &d.repeat(n),
                4 => {
                    *res += d;
                    *res += f
                }
                5 => *res += f,
                6..=8 => {
                    *res += f;
                    *res += &d.repeat(n - 5)
                }
                9 => {
                    *res += d;
                    *res += t
                }
                _ => panic!(),
            }
        }

        let mut res = String::new();

        let div_1000 = num / 1000;
        if div_1000 > 0 {
            res += &"M".repeat(div_1000 as usize);
        }

        let div_100 = ((num % 1000) / 100) as usize;
        make_roman_part(&mut res, div_100, "C", "D", "M");

        let div_10 = ((num % 100) / 10) as usize;
        make_roman_part(&mut res, div_10, "X", "L", "C");

        let div_1 = (num % 10) as usize;
        make_roman_part(&mut res, div_1, "I", "V", "X");

        res
    }
}
