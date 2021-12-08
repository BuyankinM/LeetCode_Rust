// 2094. Finding 3-Digit Even Numbers
// https://leetcode.com/problems/finding-3-digit-even-numbers/

use crate::Solution;

impl Solution {
    pub fn find_even_numbers(mut digits: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        digits.sort_unstable();

        let mut prev_x = -1;
        for (i, &x) in digits.iter().enumerate().filter(|(_, &x)| x % 2 == 0) {
            match x == prev_x {
                true => continue,
                false => prev_x = x,
            }

            let mut prev_y = -1;
            for (j, &y) in digits.iter().enumerate().filter(|(j, _)| *j != i) {
                match y == prev_y {
                    true => continue,
                    false => prev_y = y,
                }

                let mut prev_z = -1;
                for (_, &z) in digits
                    .iter()
                    .enumerate()
                    .filter(|&(k, z)| k != i && k != j && *z > 0)
                {
                    match z == prev_z {
                        true => continue,
                        false => {
                            res.push(x + 10 * y + 100 * z);
                            prev_z = z;
                        }
                    }
                }
            }
        }
        res.sort_unstable();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![102, 120, 130, 132, 210, 230, 302, 310, 312, 320],
            Solution::find_even_numbers(vec![2, 1, 3, 0])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![222, 228, 282, 288, 822, 828, 882],
            Solution::find_even_numbers(vec![2, 2, 8, 8, 2])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![0; 0], Solution::find_even_numbers(vec![3, 7, 5]));
    }

    #[test]
    fn test_4() {
        assert_eq!(vec![200], Solution::find_even_numbers(vec![0, 2, 0, 0]));
    }

    #[test]
    fn test_5() {
        assert_eq!(vec![0; 0], Solution::find_even_numbers(vec![0, 0, 0]));
    }
}
