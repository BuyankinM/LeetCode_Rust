// 1550. Three Consecutive Odds
// https://leetcode.com/problems/three-consecutive-odds/

use crate::Solution;

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut result = false;
        for wind_3 in arr.windows(3) {
            if wind_3.iter().all(|x| *x % 2 == 1) {
                result = true;
                break;
            }
        }
        result
    }

    pub fn three_consecutive_odds_optimal(arr: Vec<i32>) -> bool {
        let mut n_odds = 0;
        for is_odd in arr.iter().map(|x| *x % 2 == 1) {
            match is_odd {
                true => {
                    n_odds += 1;
                    if n_odds == 3 {
                        return true;
                    }
                }
                false => n_odds = 0,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(false, Solution::three_consecutive_odds(vec![2, 6, 4, 1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            true,
            Solution::three_consecutive_odds_optimal(vec![1, 2, 34, 3, 4, 5, 7, 23, 12])
        );
    }
}
