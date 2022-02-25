// 1002. Find Common Characters
// https://leetcode.com/problems/find-common-characters/

use crate::Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        fn get_arr_couter(word: &str) -> Vec<i32> {
            let mut v = vec![0; 26];
            word.as_bytes().iter().for_each(|&b| {
                v[(b - b'a') as usize] += 1;
            });
            v
        }

        let mut res_counter = get_arr_couter(&words[0]);
        words[1..].iter().for_each(|w| {
            let cur_counter = get_arr_couter(w);
            res_counter
                .iter_mut()
                .zip(cur_counter.iter())
                .for_each(|(res, cur)| *res = (*res).min(*cur));
        });

        res_counter
            .into_iter()
            .enumerate()
            .filter(|(_, num)| *num > 0)
            .flat_map(|(ind, num)| vec![((ind as u8 + b'a') as char).to_string(); num as usize])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec!["e".to_string(), "l".to_string(), "l".to_string()],
            Solution::common_chars(vec![
                "bella".to_string(),
                "label".to_string(),
                "roller".to_string()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec!["c".to_string(), "o".to_string()],
            Solution::common_chars(vec![
                "cool".to_string(),
                "lock".to_string(),
                "cook".to_string()
            ])
        );
    }
}
