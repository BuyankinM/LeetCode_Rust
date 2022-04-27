// 1202. Smallest String With Swaps
// https://leetcode.com/problems/smallest-string-with-swaps/

use crate::Solution;
use std::cmp::Ordering;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let p = self.parent[x];
            self.parent[x] = self.find(p);
            self.parent[x]
        }
    }

    fn union(&mut self, x: i32, y: i32) {
        let x_root = self.find(x as usize);
        let y_root = self.find(y as usize);
        if x_root == y_root {
            return;
        }

        match self.rank[x_root].cmp(&self.rank[y_root]) {
            Ordering::Less => self.parent[x_root] = y_root,
            Ordering::Greater => self.parent[y_root] = x_root,
            Ordering::Equal => {
                self.parent[x_root] = y_root;
                self.rank[y_root] += 1;
            }
        }
    }
}

impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let l = s.len();
        let sb = s.as_bytes();
        let mut uf = UnionFind::new(l);
        pairs.iter().for_each(|pair| uf.union(pair[0], pair[1]));

        let mut map = vec![vec![]; l];
        (0..l).for_each(|i| map[uf.find(i)].push(i));

        let mut res = vec![0; l];
        map.iter().filter(|idxs| !idxs.is_empty()).for_each(|idxs| {
            let mut vb = idxs.iter().map(|i| sb[*i]).collect::<Vec<_>>();
            vb.sort_unstable();
            idxs.iter().zip(vb.iter()).for_each(|(i, b)| res[*i] = *b);
        });
        String::from_utf8(res).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "bacd".to_string(),
            Solution::smallest_string_with_swaps("dcab".to_string(), vec![vec![0, 3], vec![1, 2]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "abcd".to_string(),
            Solution::smallest_string_with_swaps(
                "dcab".to_string(),
                vec![vec![0, 3], vec![1, 2], vec![0, 2]]
            )
        );
    }
}
