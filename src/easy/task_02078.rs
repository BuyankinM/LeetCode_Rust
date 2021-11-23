// 2078. Two Furthest Houses With Different Colors
// https://leetcode.com/problems/two-furthest-houses-with-different-colors/

use crate::Solution;

impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let mut used_colors = std::collections::HashSet::new();
        let mut max_dist = 0;
        let l = colors.len();

        for (i, &color_1) in colors.iter().enumerate() {
            if !used_colors.insert(color_1) {
                continue;
            }

            let j = colors[i + 1..]
                .iter()
                .rev()
                .position(|&color_2| color_1 != color_2)
                .unwrap_or(l - i - 1);

            max_dist = max_dist.max(l - j - i - 1);
            if j <= 1 {
                break;
            }
        }

        max_dist as _
    }

    // idea from - https://leetcode.com/problems/two-furthest-houses-with-different-colors/discuss/1589029/Constant-Space
    pub fn max_distance_best(colors: Vec<i32>) -> i32 {
        let mut max_dist = 0;
        let mut j = usize::MAX;
        let first_color = colors[0];

        for (&color, i) in colors[1..].iter().zip(1_usize..) {
            max_dist = match color == first_color {
                true => max_dist.max(i.saturating_sub(j)),
                false => {
                    j = j.min(i);
                    i
                }
            };
        }
        max_dist as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::max_distance(vec![1, 1, 1, 6, 1, 1, 1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, Solution::max_distance(vec![1, 8, 3, 8, 3]));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::max_distance(vec![0, 1]));
    }
}
