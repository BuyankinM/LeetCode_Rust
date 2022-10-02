// 2427. Number of Common Factors
// https://leetcode.com/problems/number-of-common-factors/

use crate::Solution;

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        use std::collections::HashSet;

        fn get_factors(n: i32) -> HashSet<i32> {
            let it = match n % 2 == 0 {
                true => (1..=n / 2).step_by(1),
                false => (1..=n / 3).step_by(2),
            };
            it.filter(|&x| n % x == 0).collect()
        }

        if a == 1 || b == 1 {
            1
        } else if a == b {
            get_factors(a).len() as i32 + 1
        } else {
            let s1 = get_factors(a);
            let s2 = get_factors(b);
            let (min, max) = match a < b {
                true => (a, b),
                false => (b, a),
            };
            s1.intersection(&s2).count() as i32 + (max % min == 0) as i32
        }
    }

    // https://leetcode.com/problems/number-of-common-factors/submissions/
    pub fn common_factors_oneliner(a: i32, b: i32) -> i32 {
        (1..=a.min(b)).filter(|f| a % f == 0 && b % f == 0).count() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::common_factors(12, 6), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::common_factors(1, 1), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::common_factors(25, 1), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::common_factors(25, 30), 2);
    }
}
