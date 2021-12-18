// 728. Self Dividing Numbers
// https://leetcode.com/problems/self-dividing-numbers/

use crate::Solution;

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut res = Vec::new();
        'outer: for num in left..=right {
            let mut n = num;
            while n > 0 {
                let rem = n % 10;
                if rem == 0 || num % rem > 0 {
                    continue 'outer;
                }
                n /= 10;
            }
            res.push(num);
        }
        res
    }

    // https://leetcode.com/problems/self-dividing-numbers/discuss/462053/Rust-Solution-in-Procedural-and-Functional
    pub fn self_dividing_numbers_func(left: i32, right: i32) -> Vec<i32> {
        (left..right + 1)
            .filter(|&x| {
                let mut n = x;
                while n != 0 {
                    let d = n % 10;
                    if d == 0 || x % d != 0 {
                        return false;
                    }
                    n /= 10;
                }
                true
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22],
            Solution::self_dividing_numbers(1, 22)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![48, 55, 66, 77],
            Solution::self_dividing_numbers(47, 85)
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![
                48, 55, 66, 77, 88, 99, 111, 112, 115, 122, 124, 126, 128, 132, 135, 144, 155, 162,
                168, 175, 184, 212, 216, 222, 224, 244, 248, 264, 288, 312, 315, 324, 333, 336,
                366, 384, 396, 412, 424, 432, 444, 448, 488, 515, 555, 612, 624, 636, 648, 666,
                672, 728, 735, 777, 784, 816, 824, 848
            ],
            Solution::self_dividing_numbers(47, 850)
        );
    }
}
