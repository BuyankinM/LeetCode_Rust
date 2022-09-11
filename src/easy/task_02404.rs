// 2404. Most Frequent Even Element
// https://leetcode.com/problems/most-frequent-even-element/

use crate::Solution;

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut max_freq = i32::MIN;
        let mut counter = std::collections::BTreeMap::new();

        nums.iter().filter(|&&x| x % 2 == 0).for_each(|&x| {
            let freq = counter.entry(x).and_modify(|n| *n += 1).or_insert(1);
            if *freq > max_freq {
                max_freq = *freq;
            }
        });

        counter
            .into_iter()
            .find(|(_, freq)| *freq == max_freq)
            .map_or(-1, |(x, _)| x)
    }

    // https://leetcode.com/problems/most-frequent-even-element/discuss/2562508/Rust-7ms-fastest-(100)-solution-with-BTreeMap-(with-detailed-comments)
    pub fn most_frequent_even_rev(nums: Vec<i32>) -> i32 {
        use std::collections::BTreeMap;

        // [1] to compute frequencies, we use BTreeMap that maintains sorted keys
        let mut freq: BTreeMap<i32, i32> = BTreeMap::new();

        // [2] populate HashMap with frequencies of even numbers
        nums.into_iter().for_each(|r| {
            if r % 2 == 0 {
                *freq.entry(r).or_default() += 1
            }
        });

        // [3] find maximum frequency among even numbers, or return -1
        freq.into_iter()
            .rev()
            .max_by_key(|&pair| pair.1)
            .unwrap_or((-1, -1))
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::most_frequent_even(vec![0, 1, 2, 2, 4, 4, 1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            -1,
            Solution::most_frequent_even(vec![29, 47, 21, 41, 13, 37, 25, 7])
        );
    }

    fn test_3() {
        assert_eq!(4, Solution::most_frequent_even(vec![4, 4, 4, 9, 2, 4]));
    }
}
