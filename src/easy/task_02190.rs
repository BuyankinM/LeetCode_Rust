// 2190. Most Frequent Number Following Key In an Array
// https://leetcode.com/problems/most-frequent-number-following-key-in-an-array/

use crate::Solution;

impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut counter = [0; 1001];
        let (mut res, mut max_c) = (0, 0);
        nums.windows(2)
            .filter_map(|pair| if pair[0] == key { Some(pair[1]) } else { None })
            .for_each(|target| {
                let c = &mut counter[target as usize];
                *c += 1;
                if *c > max_c {
                    max_c = *c;
                    res = target;
                }
            });
        res
    }

    pub fn most_frequent_one_liner(nums: Vec<i32>, key: i32) -> i32 {
        nums.windows(2)
            .filter(|pair| pair[0] == key)
            .fold([0; 1001], |mut counter, pair| {
                counter[pair[1] as usize] += 1;
                counter
            })
            .iter()
            .zip(0..)
            .fold(
                (0, 0),
                |(res, max_num), (&x, i)| if x > max_num { (i, x) } else { (res, max_num) },
            )
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(100, Solution::most_frequent(vec![1, 100, 200, 1, 100], 1));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::most_frequent(vec![2, 2, 2, 2, 3], 2));
    }
}
