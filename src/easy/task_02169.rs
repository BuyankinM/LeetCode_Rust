// 2169. Count Operations to Obtain Zero
// https://leetcode.com/problems/count-operations-to-obtain-zero/

use crate::Solution;

impl Solution {
    pub fn count_operations(mut num1: i32, mut num2: i32) -> i32 {
        let mut ops = 0;
        while num1 > 0 && num2 > 0 {
            match num1 < num2 {
                true => num2 -= num1,
                false => num1 -= num2,
            }
            ops += 1;
        }
        ops
    }

    pub fn count_operations_optimal(mut num1: i32, mut num2: i32) -> i32 {
        let mut ops = 0;
        while num1 > 0 && num2 > 0 {
            if num1 > num2 {
                std::mem::swap(&mut num1, &mut num2);
            }
            ops += num2 / num1;
            num2 %= num1;
        }
        ops
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::count_operations(2, 3));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::count_operations(10, 10));
    }

    #[test]
    fn test_3() {
        assert_eq!(10, Solution::count_operations(100, 10));
    }

    #[test]
    fn test_4() {
        assert_eq!(15, Solution::count_operations(100, 17));
    }
}
