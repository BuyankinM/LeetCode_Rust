// 1396. Design Underground System
// https://leetcode.com/problems/design-underground-system/

use std::collections::HashMap;

#[derive(Default)]
struct UndergroundSystem {
    checkins: HashMap<i32, (String, i32)>,
    travels: HashMap<(String, String), (i32, usize)>,
}

impl UndergroundSystem {
    fn new() -> Self {
        Default::default()
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.checkins.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        if let Some((start_station, start_t)) = self.checkins.get(&id) {
            let travel = self
                .travels
                .entry((start_station.clone(), station_name))
                .or_default();
            travel.0 += t - start_t;
            travel.1 += 1;
        }
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        if let Some(&(total, len)) = self.travels.get(&(start_station, end_station)) {
            return f64::from(total) / len as f64;
        }
        unreachable!()
    }
}
