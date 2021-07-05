// 1436. Destination City
// https://leetcode.com/problems/destination-city/

use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let out_going_cities: HashSet<_> = paths.iter().map(|path| &path[0]).collect();
        paths
            .iter()
            .find(|path| !out_going_cities.contains(&path[1]))
            .unwrap()[1]
            .to_owned()
    }

    pub fn dest_city_remove(mut paths: Vec<Vec<String>>) -> String {
        let set: HashSet<String> = paths.iter_mut().map(|v| v.remove(0)).collect();
        for mut v in paths {
            if !set.contains(&v[0]) {
                return v.remove(0);
            }
        }
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "Sao Paulo".to_owned(),
            Solution::dest_city(vec![
                vec!["London".to_owned(), "New York".to_owned()],
                vec!["New York".to_owned(), "Lima".to_owned()],
                vec!["Lima".to_owned(), "Sao Paulo".to_owned()]
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "A".to_owned(),
            Solution::dest_city(vec![
                vec!["B".to_owned(), "C".to_owned()],
                vec!["D".to_owned(), "B".to_owned()],
                vec!["C".to_owned(), "A".to_owned()]
            ])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "Z".to_owned(),
            Solution::dest_city(vec![vec!["A".to_owned(), "Z".to_owned()]])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            "A".to_owned(),
            Solution::dest_city_remove(vec![
                vec!["B".to_owned(), "C".to_owned()],
                vec!["D".to_owned(), "B".to_owned()],
                vec!["C".to_owned(), "A".to_owned()]
            ])
        );
    }
}
