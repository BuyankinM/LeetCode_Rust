// 1491. Average Salary Excluding the Minimum and Maximum Salary
// https://leetcode.com/problems/average-salary-excluding-the-minimum-and-maximum-salary/

use crate::Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let (mut min_sal, mut max_sal, mut sum_sal) = (std::i32::MAX, 0, 0);

        salary.iter().for_each(|&s| {
            min_sal = min_sal.min(s);
            max_sal = max_sal.max(s);
            sum_sal += s;
        });

        (sum_sal - min_sal - max_sal) as f64 / (salary.len() - 2) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2500.0, Solution::average(vec![4000,3000,1000,2000]));
    }

    #[test]
    fn test_2() {
        assert_eq!(2000.0, Solution::average(vec![1000,2000,3000]));
    }

    #[test]
    fn test_3() {
        assert_eq!(3500.0, Solution::average(vec![6000,5000,4000,3000,2000,1000]));
    }
}