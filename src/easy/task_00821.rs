// 821. Shortest Distance to a Character
// https://leetcode.com/problems/shortest-distance-to-a-character/

use crate::Solution;

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut res = Vec::with_capacity(s.len());
        let arr_pos = s
            .char_indices()
            .filter_map(|(i, cc)| if cc == c { Some(i as i32) } else { None })
            .collect::<Vec<_>>();

        let l = arr_pos.len();
        let (mut cur_pos, mut next_pos) = (arr_pos[0], arr_pos[0]);
        let (mut idx_pos, mut mid_pos) = (0, 0);

        for i in 0..s.len() as i32 {
            if i == next_pos {
                res.push(0);

                cur_pos = arr_pos[idx_pos];
                idx_pos = (idx_pos + 1).min(l - 1);
                next_pos = arr_pos[idx_pos];
                mid_pos = (next_pos + cur_pos) / 2;
                continue;
            }

            match idx_pos > 0 && idx_pos < l {
                true if i <= mid_pos => res.push((cur_pos - i).abs()),
                true => res.push((next_pos - i).abs()),
                false => res.push((cur_pos - i).abs()),
            }
        }
        res
    }

    // https://leetcode.com/problems/shortest-distance-to-a-character/discuss/694871/Rust-or-Min-Array-or-0ms
    pub fn shortest_to_char_short(s: String, c: char) -> Vec<i32> {
        let mut ci: Vec<i32> = s
            .chars()
            .enumerate()
            .filter_map(|(i, x)| if x == c { Some(i as i32) } else { None })
            .collect();
        ci.push(std::i32::MAX);

        let mut k = 0;
        let mut res = Vec::with_capacity(s.len());
        for i in 0..s.len() as i32 {
            if i - ci[k] > ci[k + 1] - i {
                k += 1;
            }
            res.push((ci[k] - i).abs());
        }
        res
    }

    // https://leetcode.com/problems/shortest-distance-to-a-character/discuss/1192984/Rust-Min-Array-(0ms)
    pub fn shortest_to_char_min_array(s: String, c: char) -> Vec<i32> {
        let len = s.len();
        let mut dist = vec![0; len];
        let mut prev = -(len as i32);

        for (i, ch) in s.char_indices() {
            if ch == c {
                prev = i as i32;
            }
            dist[i] = (i as i32 - prev) as i32;
        }

        prev = i32::MAX;
        for (i, ch) in s.char_indices().rev() {
            if ch == c {
                prev = i as i32;
            }
            dist[i] = dist[i].min(prev - (i as i32));
        }

        dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0],
            Solution::shortest_to_char("loveleetcode".to_string(), 'e')
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![3, 2, 1, 0],
            Solution::shortest_to_char("aaab".to_string(), 'b')
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 3, 2, 1, 0],
            Solution::shortest_to_char("loveleettcode".to_string(), 'e')
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            vec![3, 2, 1, 0, 1, 2, 3],
            Solution::shortest_to_char("aaabaaa".to_string(), 'b')
        );
    }
}
