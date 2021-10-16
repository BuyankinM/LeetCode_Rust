// 1122. Relative Sort Array
// https://leetcode.com/problems/relative-sort-array/

use crate::Solution;

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut counter = [0; 1001];
        let mut res = arr1;
        res.iter().for_each(|&i| counter[i as usize] += 1);

        let mut pos = 0;
        for &i in arr2.iter() {
            let num = &mut counter[i as usize];
            for _ in 0..*num {
                res[pos] = i;
                pos += 1;
            }
            *num = 0;
        }

        for (val, &n) in counter.iter().enumerate().filter(|(_, &n)| n > 0) {
            for _ in 0..n {
                res[pos] = val as i32;
                pos += 1;
            }
        }

        res
    }

    // https://leetcode.com/problems/relative-sort-array/discuss/465311/Concise-3-lines-Rust-Solution
    pub fn relative_sort_array_short_1(mut A: Vec<i32>, B: Vec<i32>) -> Vec<i32> {
        let B = B
            .iter()
            .zip(0..)
            .collect::<std::collections::HashMap<_, _>>();
        A.sort_unstable_by_key(|x| *B.get(x).unwrap_or(&(x + 1000)));
        A
    }

    // https://leetcode.com/problems/relative-sort-array/discuss/465311/Concise-3-lines-Rust-Solution
    pub fn relative_sort_array_short_2(mut A: Vec<i32>, B: Vec<i32>) -> Vec<i32> {
        let mut T = vec![2001; 1001];
        for (i, &x) in B.iter().enumerate() {
            T[x as usize] = i as i32;
        }
        A.sort_unstable_by_key(|&x| std::cmp::min(T[x as usize], x + 1000));
        A
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19],
            Solution::relative_sort_array(
                vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
                vec![2, 1, 4, 3, 9, 6]
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![22, 28, 8, 6, 17, 44],
            Solution::relative_sort_array(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![22, 28, 8, 6, 17, 44],
            Solution::relative_sort_array_short_1(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6])
        );
    }
}
