// 2287. Rearrange Characters to Make Target String
// https://leetcode.com/problems/rearrange-characters-to-make-target-string/

use crate::Solution;

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let idx = |b: u8| (b - b'a') as usize;
        let count = |s: &str| {
            s.bytes().fold([0; 26], |mut acc, b| {
                acc[idx(b)] += 1;
                acc
            })
        };

        let s_count = count(s.as_str());
        let t_count = count(target.as_str());
        t_count
            .iter()
            .zip(s_count.iter())
            .filter_map(|(&tc, &sc)| if tc > 0 { Some(sc / tc) } else { None })
            .min()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::rearrange_characters("ilovecodingonleetcode".to_string(), "code".to_string()),
            2
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::rearrange_characters("abcba".to_string(), "abc".to_string()),
            1
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::rearrange_characters("abbaccaddaeea".to_string(), "aaaaa".to_string()),
            1
        );
    }
}
