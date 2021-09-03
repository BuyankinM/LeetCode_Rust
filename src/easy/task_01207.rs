// 1207. Unique Number of Occurrences
// https://leetcode.com/problems/unique-number-of-occurrences/

use crate::Solution;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let (mut counter, mut set) = ([0; 2001], [false; 1000]);
        arr.iter().for_each(|&x| counter[(x + 1000) as usize] += 1);
        for &x in counter.iter().filter(|&&x| x > 0) {
            match set[x as usize] {
                true => return false,
                false => set[x as usize] = true,
            }
        }
        true
    }

    // https://leetcode.com/problems/unique-number-of-occurrences/discuss/462234/0ms-4-lines-Rust-Solution
    pub fn unique_occurrences_set_var1(A: Vec<i32>) -> bool {
        use std::collections::{HashMap, HashSet};
        let mut m: HashMap<i32, i32> = HashMap::new();
        A.iter().for_each(|&x| *m.entry(x).or_insert(0) += 1);
        m.values().collect::<HashSet<_>>().len() == m.len()
    }

    // https://leetcode.com/problems/unique-number-of-occurrences/discuss/635847/rust-0ms
    pub fn unique_occurrences_set_var2(arr: Vec<i32>) -> bool {
        use std::collections::{HashMap, HashSet};

        let mut occurances_map: HashMap<i32, i32> = HashMap::new();
        for num in arr {
            *occurances_map.entry(num).or_insert(0) += 1;
        }
        let mut uniq = HashSet::new();
        occurances_map.values().all(move |x| uniq.insert(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::unique_occurrences_set_var1(vec![1, 2]));
    }

    #[test]
    fn test_3() {
        assert!(Solution::unique_occurrences_set_var2(vec![
            -3, 0, 1, -3, 1, 1, 1, -3, 10, 0
        ]));
    }
}
