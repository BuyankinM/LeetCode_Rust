// 997. Find the Town Judge
// https://leetcode.com/problems/find-the-town-judge/

use crate::Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 {
            return n;
        }

        let mut counter = vec![0; (n + 1) as usize];
        trust.iter().for_each(|pair| {
            counter[pair[0] as usize] -= 1;
            counter[pair[1] as usize] += 1;
        });

        counter
            .iter()
            .position(|&num| num == n - 1)
            .map_or(-1, |i| i as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::find_judge(2, vec![vec![1, 2]]));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::find_judge(1, vec![vec![0; 0]; 0]));
    }

    #[test]
    fn test_4() {
        assert_eq!(
            3,
            Solution::find_judge(
                4,
                vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]]
            )
        );
    }
}
