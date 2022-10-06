// 981. Time Based Key-Value Store
// https://leetcode.com/problems/time-based-key-value-store/

use crate::Solution;

use std::collections::HashMap;

#[derive(Default, Debug)]
struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        Default::default()
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(v) = self.map.get(&key) {
            match v.binary_search_by_key(&timestamp, |&(time, _)| time) {
                Ok(i) => v[i].1.clone(),
                Err(i) if i > 0 => v[i - 1].1.clone(),
                _ => "".to_string(),
            }
        } else {
            "".to_string()
        }
    }

    // https://leetcode.com/problems/time-based-key-value-store/discuss/2667637/Rust-or-HashMap-and-Binary-Search-or-With-Comments
    fn get_opt(&self, key: String, timestamp: i32) -> String {
        self.map
            .get(&key)
            .and_then(|v| match v.partition_point(|&(t, _)| t <= timestamp) {
                0 => None,
                pp => Some(v[pp - 1].1.clone()),
            })
            .unwrap_or_default()
    }
}
