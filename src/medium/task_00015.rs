// 15. 3Sum
// https://leetcode.com/problems/3sum/

use crate::Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let (mut prev_x, mut prev_y) = (None, None);
        nums.sort_unstable();

        for (i, &x) in nums.iter().enumerate() {
            if prev_x == Some(x) {
                continue;
            }

            for (j, &y) in (i + 1..).zip(nums[i + 1..].iter()) {
                if prev_y == Some(y) {
                    continue;
                }

                let z = -y - x;
                if nums[j + 1..].binary_search(&z).is_ok() {
                    res.push(vec![x, y, z]);
                }
                prev_y = Some(y);
            }
            prev_x = Some(x);
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
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![vec![0; 0]; 0], Solution::three_sum(vec![0]))
    }
}
