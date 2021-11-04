// 989. Add to Array-Form of Integer
// https://leetcode.com/problems/add-to-array-form-of-integer/

use crate::Solution;

impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, mut k: i32) -> Vec<i32> {
        use std::iter::{from_fn, repeat};

        let mut q = std::collections::VecDeque::with_capacity(num.len());
        let it_num = num.into_iter().rev();
        let it_k = from_fn(move || {
            if k > 0 {
                let dig = k % 10;
                k /= 10;
                Some(dig)
            } else {
                None
            }
        });

        let mut carry = 0;

        for (dig_num, dig_k) in it_num.chain(repeat(-1)).zip(it_k.chain(repeat(-1))) {
            if dig_num == -1 && dig_k == -1 {
                break;
            }
            let sum_dig = dig_num.max(0) + dig_k.max(0) + carry;
            q.push_front(sum_dig % 10);
            carry = if sum_dig >= 10 { 1 } else { 0 };
        }

        if carry > 0 {
            q.push_front(1);
        }

        q.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![1, 2, 3, 4],
            Solution::add_to_array_form(vec![1, 2, 0, 0], 34)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![4, 5, 5],
            Solution::add_to_array_form(vec![2, 7, 4], 181)
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![1, 0, 2, 1],
            Solution::add_to_array_form(vec![2, 1, 5], 806)
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            Solution::add_to_array_form(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 1)
        );
    }
}
