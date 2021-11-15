// 739. Daily Temperatures
// https://leetcode.com/problems/daily-temperatures/

use crate::Solution;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct TempInDay {
    temp: i32,
    day: usize,
}

impl TempInDay {
    pub fn new(temp: i32, day: usize) -> Self {
        TempInDay { temp, day }
    }
}

impl Ord for TempInDay {
    fn cmp(&self, other: &Self) -> Ordering {
        other.temp.cmp(&self.temp)
    }
}

impl PartialOrd for TempInDay {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res_days = Vec::with_capacity(temperatures.len());
        for (i, &temp_cur) in temperatures.iter().enumerate() {
            let days = match temperatures[i + 1..].iter().position(|&x| x > temp_cur) {
                Some(j) => (j + 1) as i32,
                None => 0,
            };
            res_days.push(days);
        }
        res_days
    }

    pub fn daily_temperatures_heap(temperatures: Vec<i32>) -> Vec<i32> {
        let mut heap = BinaryHeap::new();
        let mut res_days = vec![0; temperatures.len()];

        for (day_cur, &temp_cur) in temperatures.iter().enumerate() {
            while let Some(TempInDay { temp, day }) = heap.peek() {
                if temp_cur <= *temp {
                    break;
                }
                res_days[*day] = (day_cur - *day) as i32;
                heap.pop();
            }
            heap.push(TempInDay::new(temp_cur, day_cur));
        }
        res_days
    }

    // https://leetcode.com/problems/daily-temperatures/discuss/1073438/Rust-solution
    pub fn daily_temperatures_stack(t: Vec<i32>) -> Vec<i32> {
        let len = t.len();
        let mut ans = vec![0; len];
        let mut stack = Vec::new();
        for curDay in 0..len {
            while stack
                .last()
                .map_or(false, |&prevDay| t[curDay] > t[prevDay])
            {
                match stack.pop() {
                    None => break,
                    Some(idx) => ans[idx] = (curDay - idx) as i32,
                }
            }
            stack.push(curDay);
        }
        ans
    }

    // solution from authors
    pub fn daily_temperatures_fast(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res_days = vec![0; temperatures.len()];
        let mut hottest = 0;

        for currDay in (0..temperatures.len()).rev() {
            let currentTemp = temperatures[currDay];
            if currentTemp >= hottest {
                hottest = currentTemp;
                continue;
            }

            let mut days = 1;
            while temperatures[currDay + days] <= currentTemp {
                // Use information from answer to search for the next warmer day
                days += res_days[currDay + days] as usize;
            }
            res_days[currDay] = days as i32;
        }
        res_days
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![1, 1, 4, 2, 1, 1, 0, 0],
            Solution::daily_temperatures_heap(vec![73, 74, 75, 71, 69, 72, 76, 73])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![1, 1, 1, 0],
            Solution::daily_temperatures_stack(vec![30, 40, 50, 60])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![1, 1, 0],
            Solution::daily_temperatures(vec![30, 60, 90])
        );
    }
}
