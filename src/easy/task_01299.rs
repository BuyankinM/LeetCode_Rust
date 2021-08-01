// 1299. Replace Elements with Greatest Element on Right Side
// https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/

use crate::Solution;

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut v = arr;
        let last_ind: usize = v.len() - 1;
        let mut max_elem = v[last_ind];
        v[last_ind] = -1;

        if last_ind == 0 {
            return v;
        }

        for i in (0..=(last_ind - 1)).rev() {
            let val = v[i];
            v[i] = max_elem;
            if val > max_elem {
                max_elem = val;
            }
        }
        v
    }

    pub fn replace_elements_short(mut A: Vec<i32>) -> Vec<i32> {
        let mut max = -1;
        for x in A.iter_mut().rev() {
            let t = *x;
            *x = max;
            max = std::cmp::max(max, t);
        }
        A
    }

    pub fn replace_elements_functional(arr: Vec<i32>) -> Vec<i32> {
        (0..(arr.len()))
            .map(|index| *arr[index + 1..].iter().max().unwrap_or(&-1))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![18, 6, 6, 6, 1, -1],
            Solution::replace_elements(vec![17, 18, 5, 4, 6, 1])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![-1], Solution::replace_elements(vec![400]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![400, 400, 400, 400, 400, 400, 400, 400, 400, 11, 11, 11, -1],
            Solution::replace_elements(vec![17, 18, 5, 4, 6, 1, 100, 200, 300, 400, 2, 3, 11])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            vec![400, 400, 400, 400, 400, 400, 400, 400, 400, 11, 11, 11, -1],
            Solution::replace_elements_functional(vec![
                17, 18, 5, 4, 6, 1, 100, 200, 300, 400, 2, 3, 11
            ])
        );
    }
}
