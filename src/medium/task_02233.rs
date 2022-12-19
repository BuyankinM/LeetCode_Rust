// 2233. Maximum Product After K Increments
// https://leetcode.com/problems/maximum-product-after-k-increments/

use crate::Solution;

impl Solution {
    pub fn maximum_product_2233(nums: Vec<i32>, k: i32) -> i32 {
        let mut k = k as i64;
        let mut heap = std::collections::BinaryHeap::with_capacity(nums.len());
        nums.iter().for_each(|&x| heap.push(-x as i64));

        match heap.len() == 1 {
            true => *heap.peek_mut().unwrap() -= k,
            false => {
                while k > 0 {
                    let x = heap.pop().unwrap();
                    let next = *heap.peek().unwrap();
                    let delta = 1.clamp(x - next, k);
                    heap.push(x - delta);
                    k -= delta;
                }
            }
        }

        let sign = if nums.len() % 2 == 0 { 1 } else { -1 };
        (heap.iter().fold(1, |prod, &x| (prod * x) % 1_000_000_007) * sign) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::maximum_product_2233(vec![92, 36, 15, 84, 57, 60, 72, 86, 70, 43, 16], 62),
            800222867
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::maximum_product_2233(vec![0, 4], 5), 20);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::maximum_product_2233(vec![6, 3, 3, 2], 2), 216);
    }
}
