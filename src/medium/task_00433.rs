// 433. Minimum Genetic Mutation
// https://leetcode.com/problems/minimum-genetic-mutation/

use crate::Solution;

use std::collections::{HashSet, VecDeque};
use std::iter::once;

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        if !bank.is_empty() {
            let bank_set = bank.into_iter().collect::<HashSet<_>>();
            let mut seen = HashSet::new();
            let mut dq = VecDeque::new();

            dq.push_back((start.clone(), 0));
            seen.insert(start);

            while let Some((gen, step)) = dq.pop_front() {
                if gen == end {
                    return step;
                }
                for c in ['A', 'C', 'G', 'T'] {
                    for i in 0..8 {
                        let new_gen = format!("{}{}{}", &gen[..i], c, &gen[i + 1..]);
                        if bank_set.contains(&new_gen) && !seen.contains(&new_gen) {
                            dq.push_back((new_gen.clone(), step + 1));
                            seen.insert(new_gen);
                        }
                    }
                }
            }
        }
        -1
    }

    // https://leetcode.com/problems/minimum-genetic-mutation/discuss/2769939/Rust-or-BFS-but-No-Hash-Set-No-Heap-Allocation-or-With-Comments
    pub fn min_mutation_optimal(start: String, end: String, mut bank: Vec<String>) -> i32 {
        let dist =
            |a: &String, b: &String| a.bytes().zip(b.bytes()).filter(|(c, d)| !c.eq(d)).count();
        let mut mutations = 0;
        let mut q = once(start).collect::<VecDeque<_>>();
        while !q.is_empty() {
            for _ in 0..q.len() {
                let curr = q.pop_front().unwrap();
                if curr == end {
                    return mutations;
                }
                let mut i = 0;
                while i < bank.len() {
                    match dist(&curr, &bank[i]) {
                        1 => q.push_back(bank.swap_remove(i)),
                        _ => i += 1,
                    }
                }
            }
            mutations += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            1,
            Solution::min_mutation(
                "AACCGGTT".to_string(),
                "AACCGGTA".to_string(),
                vec!["AACCGGTA".to_string()]
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            2,
            Solution::min_mutation(
                "AACCGGTT".to_string(),
                "AAACGGTA".to_string(),
                vec![
                    "AACCGGTA".to_string(),
                    "AACCGCTA".to_string(),
                    "AAACGGTA".to_string()
                ]
            )
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            3,
            Solution::min_mutation(
                "AAAAACCC".to_string(),
                "AACCCCCC".to_string(),
                vec![
                    "AAAACCCC".to_string(),
                    "AAACCCCC".to_string(),
                    "AACCCCCC".to_string()
                ]
            )
        );
    }
}
