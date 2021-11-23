// 832. Flipping an Image
// https://leetcode.com/problems/flipping-an-image/

use crate::Solution;

impl Solution {
    pub fn flip_and_invert_image(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let l = image[0].len();
        for row in image.iter_mut() {
            for i in 0..(l / 2 + l % 2) {
                let j = l - i - 1;
                if i < j {
                    if row[i] == row[j] {
                        row[i] ^= 1;
                        row[j] ^= 1;
                    }
                } else {
                    row[i] ^= 1;
                }
            }
        }
        image
    }

    pub fn flip_and_invert_image_1liner(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        a.into_iter()
            .map(|r| r.into_iter().map(|x| x ^ 1).rev().collect::<Vec<i32>>())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
            Solution::flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![
                vec![1, 1, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 1],
                vec![1, 0, 1, 0]
            ],
            Solution::flip_and_invert_image(vec![
                vec![1, 1, 0, 0],
                vec![1, 0, 0, 1],
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 0]
            ])
        );
    }
}
