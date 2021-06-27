// 1854. Maximum Population Year
// https://leetcode.com/problems/maximum-population-year/

use crate::Solution;

impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let (mut res_year, mut max_pop) = (0, 0);
        let mut year_pop_arr = [0; 101];
        for years in &logs {
            for year in years[0]..years[1] {
                let ind_year = (year - 1950) as usize;
                let new_pop = year_pop_arr[ind_year] + 1;
                year_pop_arr[ind_year] = new_pop;

                if new_pop > max_pop || new_pop == max_pop && res_year > year {
                    res_year = year;
                    max_pop = new_pop;
                }
            }
        }
        res_year
    }

    pub fn maximum_population_sweep_line(logs: Vec<Vec<i32>>) -> i32 {
        const RANGE: usize = 2050 - 1950 + 1;
        const YEAR_BASE: usize = 1950;

        let mut timeline: Vec<i32> = vec![0; RANGE];
        for log in logs.iter() {
            let start: usize = log[0] as usize;
            let end: usize = log[1] as usize;

            for i in start..end {
                timeline[i - YEAR_BASE] += 1;
            }
        }

        let mut max: i32 = 0;
        let mut max_year: usize = 0;

        for (idx_year, &population) in timeline.iter().enumerate() {
            if population > max {
                max = population;
                max_year = idx_year + YEAR_BASE;
            }
        }
        max_year as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            1993,
            Solution::maximum_population(vec![vec![1993, 1999], vec![2000, 2010]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            1960,
            Solution::maximum_population_sweep_line(vec![
                vec![1950, 1961],
                vec![1960, 1971],
                vec![1970, 1981]
            ])
        );
    }
}
