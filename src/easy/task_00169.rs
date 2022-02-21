// 169. Majority Element
// https://leetcode.com/problems/majority-element/

use crate::Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let l = nums.len() / 2;
        let mut m = std::collections::HashMap::with_capacity(l);
        for &n in &nums {
            let e = m.entry(n).or_insert(0);
            *e += 1;
            if *e > l {
                return n;
            }
        }
        unreachable!()
    }

    pub fn majority_element_boyer_moore(nums: Vec<i32>) -> i32 {
        let (mut counter, mut ans) = (0, nums[0]);
        for &n in &nums {
            if counter == 0 {
                ans = n;
            }
            counter += if ans == n { 1 } else { -1 }
        }
        ans
    }

    // https://leetcode.com/problems/majority-element/discuss/1702331/Rust-2-lines-0-ms
    pub fn majority_element_sort(mut nums: Vec<i32>) -> i32 {
        let l = nums.len();
        *nums.select_nth_unstable(l / 2).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::majority_element(vec![3, 2, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
    }
}
