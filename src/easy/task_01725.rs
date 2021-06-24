// 1725. Number Of Rectangles That Can Form The Largest Square
// https://leetcode.com/problems/number-of-rectangles-that-can-form-the-largest-square/

use crate::Solution;

impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let (mut res, mut max_lenght, mut length) = (0, 0, 0);

        rectangles.iter().for_each(|v| {
            length = v[0].min(v[1]);
            if length > max_lenght {
                res = 1;
                max_lenght = length;
            } else if length == max_lenght {
                res += 1;
            }
        });

        res
    }

    pub fn count_good_rectangles_map(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut hm = std::collections::HashMap::new();
        let mut max_val = 0;
        let mut max_lenght = 0;

        rectangles.iter().for_each(|v| {
            let length = v[0].min(v[1]);
            let count = hm.entry(length).or_insert(0);
            *count += 1;

            if length >= max_lenght {
                max_val = *count;
                max_lenght = length;
            }
        });

        max_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::count_good_rectangles(vec![vec![5, 8], vec![3, 9], vec![5, 12], vec![16, 5]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::count_good_rectangles_map(vec![
                vec![2, 3],
                vec![3, 7],
                vec![4, 3],
                vec![3, 7]
            ])
        );
    }
}
