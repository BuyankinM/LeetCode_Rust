// 599. Minimum Index Sum of Two Lists
// https://leetcode.com/problems/minimum-index-sum-of-two-lists/

use crate::Solution;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        use std::cmp::Ordering;
        use std::collections::HashMap;

        let map = list1.iter().zip(0..).collect::<HashMap<_, usize>>();
        let mut res = vec![];
        let mut min_sum = usize::MAX;

        for (i2, s2) in list2.into_iter().enumerate() {
            if let Some(&i1) = map.get(&s2) {
                let new_sum = i1 + i2;
                match new_sum.cmp(&min_sum) {
                    Ordering::Greater => continue,
                    Ordering::Less => {
                        min_sum = new_sum;
                        res.clear();
                    }
                    _ => (),
                }
                res.push(s2);
            }

            if i2 > min_sum {
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
            vec!["Shogun".to_string()],
            Solution::find_restaurant(
                vec![
                    "Shogun".to_string(),
                    "Tapioca Express".to_string(),
                    "Burger King".to_string(),
                    "KFC".to_string()
                ],
                vec![
                    "Piatti".to_string(),
                    "The Grill at Torrey Pines".to_string(),
                    "Hungry Hunter Steakhouse".to_string(),
                    "Shogun".to_string()
                ]
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec!["Shogun".to_string()],
            Solution::find_restaurant(
                vec![
                    "Shogun".to_string(),
                    "Tapioca Express".to_string(),
                    "Burger King".to_string(),
                    "KFC".to_string()
                ],
                vec![
                    "KFC".to_string(),
                    "Shogun".to_string(),
                    "Burger King".to_string()
                ]
            )
        );
    }
}
