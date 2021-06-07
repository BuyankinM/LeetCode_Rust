// 841. Keys and Rooms
// https://leetcode.com/problems/keys-and-rooms/
use crate::Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        
        let l = rooms.len();
        if l == 1 { return true }
        
        let mut num_vis = 1;
        let mut visited = vec![false; l];
        visited[0] = true;
        
        let mut vq = rooms[0].clone().into_iter().collect::<VecDeque<i32>>();
        
        while let Some(k) = vq.pop_front() {

            let idx = k as usize;
            
            if !visited[idx] {
                num_vis += 1;
                if num_vis == l { return true }
                
                visited[idx] = true;

                for k in rooms[idx].clone().into_iter() { 
                    vq.push_back(k);    
                }
   
            };

        }
        
        num_vis == l
    }
}