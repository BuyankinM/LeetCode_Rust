// 1640. Check Array Formation Through Concatenation
// https://leetcode.com/problems/check-array-formation-through-concatenation/

use crate::Solution;

impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let arr_ind = arr.iter().enumerate().fold([-1; 101], |mut acc, (ind, a)| {
            acc[*a as usize] = ind as i32;
            acc
        });

        for v in &pieces {
            let mut count_ind = arr_ind[v[0] as usize];
            for ind in v {
                let cur_ind = arr_ind[*ind as usize];
                if cur_ind == -1 || cur_ind != count_ind {
                    return false;
                }
                count_ind += 1;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::can_form_array(vec![85], vec![vec![85]]));
    }

    #[test]
    fn test_2() {
        assert!(Solution::can_form_array(
            vec![15, 88],
            vec![vec![88], vec![15]]
        ));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::can_form_array(
            vec![49, 18, 16],
            vec![vec![16, 18, 49]]
        ));
    }

    #[test]
    fn test_4() {
        assert!(Solution::can_form_array(
            vec![91, 4, 64, 78],
            vec![vec![78], vec![4, 64], vec![91]]
        ));
    }
}
