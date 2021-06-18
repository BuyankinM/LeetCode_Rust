// 1608. Special Array With X Elements Greater Than or Equal X
// https://leetcode.com/problems/special-array-with-x-elements-greater-than-or-equal-x/

use crate::Solution;

impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let arr_count = nums.iter().fold([0; 1001], |mut acc, &x| {
            acc[x as usize] += 1;
            acc
        });
        let mut prev_sum = 0;
        let l = nums.len() as i32;

        for i in 0..=l {
            let after_sum = l - prev_sum;
            prev_sum += arr_count[i as usize];

            if after_sum == i {
                return i;
            } else if prev_sum == l {
                break;
            }
        }

        -1
    }

    pub fn special_array_optimal(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut count = vec![0; n + 1];
        for &x in &nums {
            if x >= n as i32 {
                count[n] += 1;
            } else {
                count[x as usize] += 1;
            }
        }
        let mut ans = 0;
        for i in (1..=n).rev() {
            ans += count[i];
            if ans == i {
                return ans as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::special_array(vec![3, 5]));
    }

    #[test]
    fn test_2() {
        assert_eq!(-1, Solution::special_array(vec![0, 0]));
    }

    #[test]
    fn test_3() {
        assert_eq!(3, Solution::special_array_optimal(vec![0, 4, 3, 0, 4]));
    }
}
