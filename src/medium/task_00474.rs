// 474. Ones and Zeroes
// https://leetcode.com/problems/ones-and-zeroes/

use crate::Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let (mu, nu) = (m as usize, n as usize);
        let mut dp = vec![vec![0; nu + 1]; mu + 1];
        for s in strs {
            let (mut zeros, mut ones) = (0, 0);
            s.chars().for_each(|c| match c == '0' {
                true => zeros += 1,
                false => ones += 1,
            });
            for i in (zeros..=mu).rev() {
                for j in (ones..=nu).rev() {
                    dp[i][j] = dp[i][j].max(dp[i - zeros][j - ones] + 1);
                }
            }
        }
        dp[mu][nu]
    }

    pub fn find_max_form_recursive(strs: Vec<String>, m: i32, n: i32) -> i32 {
        fn top_down_dp(
            dp: &mut [[[i32; 101]; 101]; 601],
            c: &[(usize, usize)],
            idx: usize,
            zeros: usize,
            ones: usize,
            max_zeros: usize,
            max_ones: usize,
        ) -> i32 {
            if idx == c.len() || zeros > max_zeros || ones > max_ones {
                return 0;
            }

            if dp[idx][zeros][ones] > 0 {
                return dp[idx][zeros][ones];
            }

            let (cur_zeros, cur_ones) = c[idx];
            let (mut include, mut exclude, mut without) = (0, 0, 0);
            let (new_zeros, new_ones) = (zeros + cur_zeros, ones + cur_ones);

            if new_zeros <= max_zeros && new_ones <= max_ones {
                include = 1 + top_down_dp(dp, c, idx + 1, new_zeros, new_ones, max_zeros, max_ones);
                exclude = top_down_dp(dp, c, idx + 1, zeros, ones, max_zeros, max_ones);
            } else {
                without = top_down_dp(dp, c, idx + 1, zeros, ones, max_zeros, max_ones);
            }

            let max = include.max(exclude).max(without);
            dp[idx][zeros][ones] = max;
            max
        }

        let mut dp = [[[-1; 101]; 101]; 601];
        let counter = strs
            .iter()
            .map(|s| {
                let zeros = s.chars().filter(|&c| c == '0').count();
                let ones = s.len() - zeros;
                (zeros, ones)
            })
            .collect::<Vec<_>>();

        top_down_dp(&mut dp, &counter, 0, 0, 0, m as usize, n as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_max_form(
                vec![
                    "10".to_string(),
                    "0001".to_string(),
                    "111001".to_string(),
                    "1".to_string(),
                    "0".to_string()
                ],
                5,
                3
            ),
            4
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::find_max_form(
                vec!["10".to_string(), "1".to_string(), "0".to_string()],
                1,
                1
            ),
            2
        );
    }
}
