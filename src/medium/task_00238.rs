// 238. Product of Array Except Self
// https://leetcode.com/problems/product-of-array-except-self/

use crate::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let l = nums.len();
        let mut ans = Vec::with_capacity(l);

        let mut prev = nums[0];
        ans.push(prev);

        nums[1..l - 1].iter().for_each(|&x| {
            prev *= x;
            ans.push(prev)
        });

        ans.push(prev); // last element
        prev = nums[l - 1];

        (1..l - 1).rev().for_each(|i| {
            ans[i] = prev * ans[i - 1];
            prev *= nums[i];
        });
        ans[0] = prev;

        ans
    }

    // https://leetcode.com/problems/product-of-array-except-self/discuss/1598363/Rust-solution
    pub fn product_except_self_func(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<_> = nums
            .iter()
            .scan(1, |s, x| Some(std::mem::replace(s, *s * x)))
            .collect();
        nums.iter()
            .rev()
            .scan(1, |s, x| Some(std::mem::replace(s, *s * x)))
            .zip(ans.iter_mut().rev())
            .for_each(|(a, b)| *b *= a);
        ans
    }

    // https://leetcode.com/problems/product-of-array-except-self/discuss/1566288/Rust-using-iterators
    pub fn product_except_self_chain(nums: Vec<i32>) -> Vec<i32> {
        use std::iter::once;

        let n = nums.len();
        let mut left_prod = 1;
        let prod_left = once(left_prod).chain(nums.iter().take(n - 1).map(|num| {
            left_prod *= *num;
            left_prod
        }));
        let mut right_prod = 1;
        let prod_right = once(right_prod)
            .chain(nums.iter().skip(1).rev().map(|num| {
                right_prod *= *num;
                right_prod
            }))
            .collect::<Vec<_>>()
            .into_iter()
            .rev();
        prod_left
            .zip(prod_right)
            .map(|(left, right)| left * right)
            .collect()
    }

    // https://leetcode.com/problems/product-of-array-except-self/discuss/520933/Rust-one-loop-only
    pub fn product_except_self_one_loop(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![1; nums.len()];
        let mut left = 1;
        let mut right = 1;
        for i in 1..nums.len() {
            left *= nums[i - 1];
            res[i] *= left;

            right *= nums[nums.len() - i];
            res[nums.len() - i - 1] *= right;
        }

        res
    }

    // https://leetcode.com/problems/product-of-array-except-self/discuss/498124/Rust-O(n)-constant-space-solution
    pub fn product_except_self_short(nums: Vec<i32>) -> Vec<i32> {
        let mut answer: Vec<i32> = vec![1; nums.len()];
        let mut n = 1;
        for i in 0..nums.len() {
            answer[i] *= n;
            n *= nums[i];
        }
        n = 1;
        for i in (0..nums.len()).rev() {
            answer[i] *= n;
            n *= nums[i];
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![24, 12, 8, 6],
            Solution::product_except_self(vec![1, 2, 3, 4])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![0, 0, 9, 0, 0],
            Solution::product_except_self(vec![-1, 1, 0, -3, 3])
        );
    }
}
