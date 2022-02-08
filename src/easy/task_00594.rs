// 594. Longest Harmonious Subsequence
// https://leetcode.com/problems/longest-harmonious-subsequence/

use crate::Solution;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut btm = std::collections::BTreeMap::new();
        nums.iter().for_each(|&x| *btm.entry(x).or_insert(0) += 1);

        let (mut prev_num, mut prev_len, mut max_len) = (None, 0, 0);
        btm.into_iter().for_each(|(cur_num, cur_len)| {
            if let Some(num) = prev_num {
                if cur_num - num == 1 {
                    max_len = max_len.max(cur_len + prev_len);
                }
            }
            prev_num = Some(cur_num);
            prev_len = cur_len;
        });
        max_len
    }

    // https://leetcode.com/problems/longest-harmonious-subsequence/discuss/1049196/Rust-HashMap-solution
    pub fn find_lhs_hashmap(nums: Vec<i32>) -> i32 {
        let mut hm = std::collections::HashMap::new();
        nums.iter()
            .for_each(|&num| *hm.entry(num).or_insert(0) += 1);
        hm.iter().fold(0, |acc, (&num, &count)| {
            hm.get(&(num + 1)).map_or(acc, |c| acc.max(count + c))
        })
    }

    // https://leetcode.com/problems/longest-harmonious-subsequence/discuss/1049731/Rust%3A-16ms-100-HashMap-Solution-Using-Iterator-Methods-O(n)
    pub fn find_lhs_hashmap_2(nums: Vec<i32>) -> i32 {
        let mut counts = std::collections::HashMap::new();
        nums.into_iter()
            .for_each(|x| *counts.entry(x).or_insert(0) += 1);
        counts
            .iter()
            .filter_map(|(&k, &v)| counts.get(&(k + 1)).map(|n| n + v))
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::find_lhs(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::find_lhs(vec![1, 1, 1, 1]));
    }
}
