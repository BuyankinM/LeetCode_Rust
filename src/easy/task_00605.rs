// 605. Can Place Flowers
// https://leetcode.com/problems/can-place-flowers/

use crate::Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut n = n;
        let mut num_free = 0;
        for &plot in &flowerbed {
            match plot {
                1 => {
                    num_free -= 1; // correct prev free plots
                    n -= (num_free + 1) / 2;
                    if n <= 0 {
                        return true;
                    }
                    num_free = -1; // correct next free plots
                }
                _ => num_free += 1,
            }
        }
        if num_free > 0 {
            n -= (num_free + 1) / 2;
        }
        n <= 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::can_place_flowers(vec![1, 0, 0, 1], 1));
    }

    #[test]
    fn test_3() {
        assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 0], 2));
    }

    #[test]
    fn test_4() {
        assert!(Solution::can_place_flowers(vec![0, 0, 0, 0, 0], 3));
    }
}
