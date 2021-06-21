// 1700. Number of Students Unable to Eat Lunch
// https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/

use crate::Solution;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut counter = students.iter().fold([0; 2], |mut acc, &x| {
            acc[x as usize] += 1;
            acc
        });

        for s in sandwiches {
            let ind = s as usize;
            match counter[ind] > 0 {
                true => counter[ind] -= 1,
                false => break,
            }
        }

        counter[0] + counter[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            0,
            Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1])
        );
    }
}
