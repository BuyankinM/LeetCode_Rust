// 67. Add Binary
// https://leetcode.com/problems/add-binary/

use crate::Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        use std::iter;
        let mut carry = 0;
        let mut cur_sum = 0;
        
        let mut char_vec = a
            .as_bytes()
            .iter()
            .rev()
            .chain(iter::repeat(&b'0'))
            .zip(b.as_bytes().iter().rev().chain(iter::repeat(&b'0')))
            .take(a.len().max(b.len()))
            .map(|(ac, bc)| {
                cur_sum = (*ac - b'0') + (*bc - b'0') + carry;
                carry = cur_sum / 2;
                match cur_sum % 2 {
                    1 => '1',
                    _ => '0',
                }
            })
            .collect::<Vec<_>>();

        if carry == 1 {
            char_vec.push('1');
        }

        char_vec.iter().rev().collect()
    }

    pub fn add_binary_short(a: String, b: String) -> String {
        format!(
            "{:b}",
            u128::from_str_radix(&a, 2).unwrap() + u128::from_str_radix(&b, 2).unwrap()
        )
    }

    pub fn add_binary_string(mut a: String, mut b: String) -> String {
        if a.len() < b.len() {
            std::mem::swap(&mut a, &mut b);
        }

        let mut s = String::new();
        let mut sum = 0;
        let mut chars = b.chars().rev();
        for c in a.chars().rev() {
            if c == '1' {
                sum += 1;
            }
            if let Some('1') = chars.next() {
                sum += 1;
            }
            s.insert(0, if sum & 1 == 1 { '1' } else { '0' });
            sum >>= 1;
        }
        if sum != 0 {
            s.insert(0, '1');
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "100".to_owned(),
            Solution::add_binary("11".to_owned(), "1".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "10101".to_owned(),
            Solution::add_binary("1011".to_owned(), "1010".to_owned())
        );
    }
}
