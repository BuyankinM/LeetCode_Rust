// 2383. Minimum Hours of Training to Win a Competition
// https://leetcode.com/problems/minimum-hours-of-training-to-win-a-competition/

use crate::Solution;

impl Solution {
    pub fn min_number_of_hours(
        initial_energy: i32,
        initial_experience: i32,
        energy: Vec<i32>,
        experience: Vec<i32>,
    ) -> i32 {
        let delta_energy = 0.max(energy.iter().sum::<i32>() - initial_energy + 1);
        let (mut delta_exp, mut exp) = (0, initial_experience);
        experience.iter().for_each(|&e| match e < exp {
            true => exp += e,
            false => {
                delta_exp += e - exp + 1;
                exp = 2 * e + 1;
            }
        });

        delta_energy + delta_exp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::min_number_of_hours(5, 3, vec![1, 4, 3, 2, 5], vec![2, 6, 3, 1, 99]),
            96
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::min_number_of_hours(5, 3, vec![1, 4, 3, 2], vec![2, 6, 3, 1]),
            8
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::min_number_of_hours(2, 4, vec![1], vec![3]), 0);
    }
}
