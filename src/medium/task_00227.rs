// 227. Basic Calculator II
// https://leetcode.com/problems/basic-calculator-ii/

use crate::Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        use std::collections::HashMap;

        let mut num = 0;
        let mut nums = Vec::new();
        let mut ops = Vec::new();
        let priorities: HashMap<char, u8> = [('+', 1), ('-', 1), ('*', 2), ('/', 2)]
            .iter()
            .cloned()
            .collect();
        let calc = |a: i32, b: i32, op: char| -> i32 {
            match op {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                _ => a / b,
            }
        };

        s.chars().for_each(|c| match c {
            '0'..='9' => num = (10 * num) + c.to_digit(10).unwrap() as i32,
            '+' | '-' | '/' | '*' => {
                nums.push(num);
                while let Some(last_op) = ops.last() {
                    if nums.len() == 1 || priorities[&c] > priorities[last_op] {
                        break;
                    }
                    let op = ops.pop().unwrap();
                    let (b, a) = (nums.pop().unwrap(), nums.pop().unwrap());
                    nums.push(calc(a, b, op));
                }
                ops.push(c);
                num = 0;
            }
            _ => (),
        });
        nums.push(num);

        while let Some(op) = ops.pop() {
            let (b, a) = (nums.pop().unwrap(), nums.pop().unwrap());
            nums.push(calc(a, b, op));
        }
        nums[0]
    }

    // https://leetcode.com/problems/basic-calculator-ii/discuss/736602/Rust-cheapest-and-best
    pub fn calculate_fast(mut s: String) -> i32 {
        s.push('.');
        s.chars()
            .fold((vec![], 0i32, '+'), |(mut s, mut n, mut op), c| {
                if c == ' ' {
                    return (s, n, op);
                }
                match c.to_digit(10) {
                    Some(digit) => n = n * 10 + digit as i32,
                    _ => {
                        match op {
                            '+' => s.push(n),
                            '-' => s.push(-n),
                            '/' => *s.last_mut().unwrap() /= n,
                            '*' => *s.last_mut().unwrap() *= n,
                            _ => {}
                        }
                        op = c;
                        n = 0;
                    }
                }
                (s, n, op)
            })
            .0
            .iter()
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(7, Solution::calculate("3+2*2".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::calculate(" 3/2 ".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(5, Solution::calculate(" 3+5 / 2 ".to_string()));
    }

    #[test]
    fn test_4() {
        assert_eq!(10, Solution::calculate(" 3+5 +   2 ".to_string()));
    }

    #[test]
    fn test_5() {
        assert_eq!(10, Solution::calculate(" 2 * 3 + 4 ".to_string()));
    }

    #[test]
    fn test_6() {
        assert_eq!(
            -24,
            Solution::calculate("1*2 - 3/4 + 5*6 - 7*8 + 9/10".to_string())
        );
    }

    #[test]
    fn test_7() {
        assert_eq!(9, Solution::calculate("2 - 0 + 7".to_string()));
    }

    #[test]
    fn test_8() {
        assert_eq!(6, Solution::calculate("1 + 2*5/3 + 6/4*2".to_string()));
    }
}
