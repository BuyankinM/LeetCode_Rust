// 1331. Rank Transform of an Array
// https://leetcode.com/problems/rank-transform-of-an-array/

use crate::Solution;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        use std::collections::BTreeMap;

        let mut BTmap = arr.iter().map(|x| (*x, 0)).collect::<BTreeMap<_, _>>();
        BTmap
            .iter_mut()
            .zip(1..)
            .for_each(|((_, value), num)| *value = num);

        arr.iter().map(|x| BTmap[x]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![4, 1, 2, 3],
            Solution::array_rank_transform(vec![40, 10, 20, 30])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![1, 1, 1],
            Solution::array_rank_transform(vec![100, 100, 100])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![5, 3, 4, 2, 8, 6, 7, 1, 3],
            Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12])
        );
    }
}
