// 2073. Time Needed to Buy Tickets
// https://leetcode.com/problems/time-needed-to-buy-tickets/

use crate::Solution;

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let num_k = tickets[k as usize];
        tickets.iter().zip(0..).fold(0, |s, (&x, i)| {
            s + match i <= k {
                true => x.min(num_k),
                false => x.min(num_k - 1),
            }
        })
    }

    pub fn time_required_to_buy_short(tickets: Vec<i32>, k: i32) -> i32 {
        let num_k = tickets[k as usize];
        tickets
            .iter()
            .zip(0..)
            .fold(0, |s, (&x, i)| s + x.min(num_k - (k > i) as i32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::time_required_to_buy(vec![2, 3, 2], 2));
    }

    #[test]
    fn test_2() {
        assert_eq!(8, Solution::time_required_to_buy(vec![5, 1, 1, 1], 0));
    }
}
