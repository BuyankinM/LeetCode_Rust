// 1450. Number of Students Doing Homework at a Given Time
// https://leetcode.com/problems/number-of-students-doing-homework-at-a-given-time/

use crate::Solution;

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        start_time
            .iter()
            .zip(end_time.iter())
            .filter(|(start, end)| **start <= query_time && **end >= query_time)
            .count() as i32
    }
    pub fn busy_student_without_zip(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        start_time
            .iter()
            .zip(end_time.iter())
            .filter(|(start, end)| **start <= query_time && **end >= query_time)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::busy_student(vec![1,2,3], vec![3,2,7], 4));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::busy_student(vec![4], vec![4], 4));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::busy_student(vec![1,1,1,1], vec![1,3,2,4], 7));
    }
    
    #[test]
    fn test_4() {
        assert_eq!(5, Solution::busy_student(vec![9,8,7,6,5,4,3,2,1], vec![10,10,10,10,10,10,10,10,10], 5));
    }
}