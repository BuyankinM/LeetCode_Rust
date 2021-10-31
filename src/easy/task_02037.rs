// 2037. Minimum Number of Moves to Seat Everyone
// https://leetcode.com/problems/minimum-number-of-moves-to-seat-everyone/

use crate::Solution;

impl Solution {
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort_unstable();
        students.sort_unstable();
        seats
            .iter()
            .zip(students.iter())
            .map(|(seat, stud)| (seat - stud).abs())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4]));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            7,
            Solution::min_moves_to_seat(vec![4, 1, 5, 9], vec![1, 3, 2, 6])
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            4,
            Solution::min_moves_to_seat(vec![2, 2, 6, 6], vec![1, 3, 2, 6])
        );
    }
}
