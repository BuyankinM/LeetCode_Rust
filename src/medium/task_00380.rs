// 380. Insert Delete GetRandom O(1)
// https://leetcode.com/problems/insert-delete-getrandom-o1/
use rand::{thread_rng, Rng};
use std::collections::HashMap;

#[derive(Default)]
struct RandomizedSet {
    list: Vec<i32>,
    map: HashMap<i32, usize>,
}

impl RandomizedSet {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }

        self.map.insert(val, self.list.len());
        self.list.push(val);

        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.map.contains_key(&val) {
            return false;
        }

        let cur_ind = *self.map.get(&val).unwrap();
        let last_val = *self.list.last().unwrap();
        
        self.map.insert(last_val, cur_ind);
        self.list.swap_remove(cur_ind);
        self.map.remove(&val);

        true
    }

    fn get_random(&mut self) -> i32 {
        let mut rng = thread_rng();
        self.list[rng.gen_range(0..self.list.len())]
    }
}

// /**
//  * Your RandomizedSet object will be instantiated and called as such:
//  * let obj = RandomizedSet::new();
//  * let ret_1: bool = obj.insert(val);
//  * let ret_2: bool = obj.remove(val);
//  * let ret_3: i32 = obj.get_random();
//  */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut obj = RandomizedSet::new();
        let ret_1: bool = obj.insert(0);
        let ret_2: bool = obj.insert(1);
        let ret_3: bool = obj.remove(0);
        let ret_4: bool = obj.insert(2);
        let ret_5: bool = obj.remove(1);
        let ret_6: i32 = obj.get_random();

        assert!(ret_1);
        assert!(ret_2);
        assert!(ret_3);
        assert!(ret_4);
        assert!(ret_5);
        assert_eq!(2, ret_6);
    }
}
