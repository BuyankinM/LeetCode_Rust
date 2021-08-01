// 1496. Path Crossing
// https://leetcode.com/problems/path-crossing/

use crate::Solution;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut path_hash = std::collections::HashSet::new();
        let mut cur_point = (0, 0);
        for c in path.chars() {
            path_hash.insert(cur_point.clone());

            match c {
                'N' => cur_point = (cur_point.0, cur_point.1 + 1),
                'S' => cur_point = (cur_point.0, cur_point.1 - 1),
                'E' => cur_point = (cur_point.0 + 1, cur_point.1),
                'W' => cur_point = (cur_point.0 - 1, cur_point.1),
                _ => (),
            }

            if path_hash.contains(&cur_point) {
                return true;
            }
        }
        false
    }

    pub fn is_path_crossing_xy(path: String) -> bool {
        let mut path_hash = std::collections::HashSet::new();
        let (mut x, mut y) = (0, 0);

        for c in path.chars() {
            path_hash.insert((x, y));

            match c {
                'N' => y += 1,
                'S' => y -= 1,
                'E' => x += 1,
                _ => x -= 1, // W
            }

            if path_hash.contains(&(x, y)) {
                return true;
            }
        }
        false
    }

    pub fn is_path_crossing_best(path: String) -> bool {
        use std::collections::HashSet;

        let mut point = 0; // point = x * k + y
        let k = 15_000; // must be > 10_000

        let mut path_hash = HashSet::with_capacity(path.len() + 1);
        path_hash.insert(point);

        for c in path.chars() {
            match c {
                'N' => point += 1, // y + 1
                'S' => point -= 1, // y - 1
                'E' => point += k, // x + 1
                _ => point -= k,   // x - 1
            }

            match path_hash.contains(&point) {
                true => return true,
                false => path_hash.insert(point),
            };
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(false, Solution::is_path_crossing("NES".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(true, Solution::is_path_crossing_xy("NESWW".to_owned()));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            true,
            Solution::is_path_crossing_best("NESWEWEWEWESNSN".to_owned())
        );
    }
}
