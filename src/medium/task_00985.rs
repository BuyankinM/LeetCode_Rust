// 985. Sum of Even Numbers After Queries
// https://leetcode.com/problems/sum-of-even-numbers-after-queries/

use crate::Solution;

impl Solution {
    pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut even_sum = nums.iter().filter(|&&x| x % 2 == 0).sum();
        let mut res = Vec::with_capacity(queries.len());
        queries.iter().for_each(|pair| {
            if let &[val, ind] = pair.as_slice() {
                let prev_val = &mut nums[ind as usize];
                even_sum += match (*prev_val % 2 == 0, val % 2 == 0) {
                    (true, val_even) => match val_even {
                        true => val,
                        false => -(*prev_val),
                    },
                    (false, false) => *prev_val + val,
                    _ => 0,
                };
                *prev_val += val;
            }
            res.push(even_sum);
        });
        res
    }

    // https://leetcode.com/problems/sum-of-even-numbers-after-queries/discuss/2604371/Rust-or-Update-Sum-or-With-Comments
    pub fn sum_even_after_queries_opt(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sum: i32 = nums.iter().filter(|num| **num % 2 == 0).sum();
        let mut rez = vec![];
        let mut it = queries.iter().map(|query| query.as_slice());
        while let Some(&[val, index]) = it.next() {
            let index = index as usize;
            if nums[index] % 2 == 0 {
                sum -= nums[index]
            }
            nums[index] += val;
            if nums[index] % 2 == 0 {
                sum += nums[index]
            }
            rez.push(sum);
        }
        rez
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::sum_even_after_queries(vec![1], vec![vec![4, 0]]),
            vec![0]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::sum_even_after_queries(
                vec![1, 2, 3, 4],
                vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]]
            ),
            vec![8, 6, 2, 4]
        );
    }
}
