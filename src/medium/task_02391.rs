// 2391. Minimum Amount of Time to Collect Garbage
// https://leetcode.com/problems/minimum-amount-of-time-to-collect-garbage/

use crate::Solution;

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let travel_acc = std::iter::once(0)
            .chain(travel.iter().scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            }))
            .collect::<Vec<_>>();

        let mut counter_map = [(0, 0); 3];
        for (i, house_garbage) in garbage.iter().enumerate() {
            for g_type in house_garbage.chars() {
                let idx = match g_type {
                    'G' => 0,
                    'P' => 1,
                    _ => 2,
                };
                let elem = &mut counter_map[idx];
                elem.0 = i;
                elem.1 += 1;
            }
        }

        counter_map.iter().fold(0, |acc, &(last_house, total)| {
            acc + total + travel_acc[last_house]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::garbage_collection(
                vec![
                    "G".to_string(),
                    "P".to_string(),
                    "GP".to_string(),
                    "GG".to_string()
                ],
                vec![2, 4, 3]
            ),
            21
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::garbage_collection(
                vec!["MMM".to_string(), "PGM".to_string(), "GP".to_string()],
                vec![3, 10]
            ),
            37
        );
    }
}
