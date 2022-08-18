// 1338. Reduce Array Size to The Half
// https://leetcode.com/problems/reduce-array-size-to-the-half/

use crate::Solution;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut counter = vec![0; 100_001];
        let mut l = arr.len();
        let half = l / 2;

        arr.iter().for_each(|&x| counter[x as usize] += 1);
        counter.sort_unstable_by_key(|&x| -x);

        let mut res = 0;
        for n in counter {
            l -= n as usize;
            res += 1;
            if l <= half {
                break;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::min_set_size(vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7]),
            2
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::min_set_size(vec![7, 7, 7, 7, 7, 7]), 1);
    }
}
