// 977. Squares of a Sorted Array
// https://leetcode.com/problems/squares-of-a-sorted-array/

use crate::Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut v: Vec<i32> = nums.iter().map(|x| x * x).collect();
        v.sort_unstable();
        v
    }

    pub fn sorted_squares_ON(nums: Vec<i32>) -> Vec<i32> {
        use std::cmp::Ordering;

        let l = nums.len();
        let mut arr_square = vec![0; l];

        if l == 1 {
            arr_square[0] = nums[0].pow(2);
        } else if nums[0] >= 0 {
            arr_square = nums.iter().map(|x| x.pow(2)).collect();
        } else if nums[l - 1] <= 0 {
            arr_square = nums.iter().rev().map(|x| x.pow(2)).collect();
        } else {
            let mut i: usize = 0;
            let mut j: usize = l - 1;
            let mut k: usize = l - 1;

            while i < j {
                let n_i = nums[i].pow(2);
                let n_j = nums[j].pow(2);

                match n_i.cmp(&n_j) {
                    Ordering::Greater => {
                        arr_square[k] = n_i;
                        i += 1;
                    }
                    Ordering::Less => {
                        arr_square[k] = n_j;
                        j -= 1;
                    }
                    Ordering::Equal => {
                        arr_square[k] = n_j;
                        arr_square[k - 1] = n_i;
                        j -= 1;
                        i += 1;
                        k -= 1
                    }
                }
                k -= 1;
            }
            if i == j {
                arr_square[0] = nums[i].pow(2)
            }
        }
        arr_square
    }

    pub fn sorted_squares_ON_short(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let (mut i, mut j, mut p) = (0, nums.len() - 1, nums.len() - 1);
        loop {
            if nums[i] + nums[j] <= 0 {
                res[p] = nums[i] * nums[i];
                i += 1;
            } else {
                res[p] = nums[j] * nums[j];
                match j == 0 {
                    false => j -= 1,
                    true => break,
                }
            }
            match p == 0 {
                false => p -= 1,
                true => break,
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![0, 1, 9, 16, 100],
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![4, 9, 9, 49, 121],
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![0, 1, 9, 16, 100],
            Solution::sorted_squares_ON(vec![-4, -1, 0, 3, 10])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            vec![4, 9, 9, 49, 121],
            Solution::sorted_squares_ON_short(vec![-7, -3, 2, 3, 11])
        );
    }
}
