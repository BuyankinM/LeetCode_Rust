// 1528. Shuffle String
// https://leetcode.com/problems/shuffle-string/

use crate::Solution;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        s.chars()
            .zip(indices)
            .fold(vec!['a'; s.len()], |mut v, (c, ind)| {
                v[ind as usize] = c;
                v
            })
            .iter()
            .collect()
    }

    pub fn restore_string_1(s: String, indices: Vec<i32>) -> String {
        let mut out = vec![' '; s.len()];

        for i in 0..indices.len() {
            out[indices[i] as usize] = s.chars().nth(i).unwrap();
        }

        return out.into_iter().collect();
    }

    pub fn restore_string_2(s: String, indices: Vec<i32>) -> String {
        let mut v = vec![' '; s.len()];
        s.chars().zip(indices).for_each(|(c, ind)| {
            v[ind as usize] = c;
        });
        v.into_iter().collect()
    }

    pub fn restore_string_cyclic_sort(s: String, mut indices: Vec<i32>) -> String {
        let mut v = s.into_bytes();
        for i in 0..v.len() {
            while indices[i] != i as i32 {
                let new_ind = indices[i] as usize;
                v.swap(i, new_ind);
                indices.swap(i, new_ind);
            }
        }
        String::from_utf8(v).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "abc".to_owned(),
            Solution::restore_string("abc".to_owned(), vec![0, 1, 2])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "nihao".to_owned(),
            Solution::restore_string_1("aiohn".to_owned(), vec![3, 1, 4, 2, 0])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "arigatou".to_owned(),
            Solution::restore_string_2("aaiougrt".to_owned(), vec![4, 0, 2, 6, 7, 3, 1, 5])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            "rat".to_owned(),
            Solution::restore_string_cyclic_sort("art".to_owned(), vec![1, 0, 2])
        );
    }
}
