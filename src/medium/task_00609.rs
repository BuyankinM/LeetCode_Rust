// 609. Find Duplicate File in System
// https://leetcode.com/problems/find-duplicate-file-in-system/

use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn find_duplicate_medium_609(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut hm: HashMap<&str, Vec<String>> = HashMap::with_capacity(paths.len());
        for path in paths.iter() {
            let pos_sep = path.find(' ').unwrap();
            let dir = &path[..pos_sep];

            for name_cont in path[pos_sep..].split_ascii_whitespace() {
                let mut file_it = name_cont.split(&['(', ')'][..]);
                let name = file_it.next().unwrap();
                let content = file_it.next().unwrap();
                hm.entry(content).or_default().push(format!("{dir}/{name}"));
            }
        }

        hm.into_values().filter(|v| v.len() > 1).collect()
    }

    // https://leetcode.com/problems/find-duplicate-file-in-system/discuss/1215511/Rust-HashMap-solution
    pub fn find_duplicate_medium_609_short(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut hm = HashMap::new();
        for input in &paths {
            let v = input.split(' ').collect::<Vec<_>>();
            for &file in &v[1..] {
                if let Some(pos) = file.chars().position(|c| c == '(') {
                    hm.entry(&file[pos + 1..file.len() - 1])
                        .or_insert_with(Vec::new)
                        .push(String::new() + v[0] + "/" + &file[..pos]);
                }
            }
        }
        hm.into_iter()
            .filter_map(|(_, v)| if v.len() > 1 { Some(v) } else { None })
            .collect()
    }

    // https://leetcode.com/problems/find-duplicate-file-in-system/discuss/2596128/Rust-or-HashMap-or-With-Comments
    pub fn find_duplicate_medium_609_optimal(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::<&str, Vec<(&str, &str)>>::new();
        for entry in &paths {
            let mut entry_parts = entry.split(&[' ', '(', ')']).filter(|s| !s.is_empty());
            let path = entry_parts.next().unwrap();
            while let (Some(name), Some(content)) = (entry_parts.next(), entry_parts.next()) {
                map.entry(content).or_default().push((path, name));
            }
        }

        map.into_values()
            .filter_map(|files| {
                (files.len() > 1).then(|| {
                    files
                        .into_iter()
                        .map(|(path, name)| format!("{path}/{name}"))
                        .collect::<Vec<_>>()
                })
            })
            .collect()
    }
}
