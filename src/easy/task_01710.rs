// 1710. Maximum Units on a Truck
// https://leetcode.com/problems/maximum-units-on-a-truck/

use crate::Solution;

impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_unstable_by(|a, b| b[1].cmp(&a[1]));
        let mut units = 0;

        for v in box_types {
            let num_boxes = truck_size.min(v[0]);
            units += num_boxes * v[1];
            truck_size -= num_boxes;
            if truck_size == 0 {
                break;
            }
        }

        units
    }

    pub fn maximum_units_scan(mut b: Vec<Vec<i32>>, t: i32) -> i32 {
        b.sort_by_key(|x| -x[1]);
        b.into_iter()
            .scan(t, |s, p| {
                let (b, u) = (p[0].min(*s), p[1]);
                *s -= b;
                Some(b * u)
            })
            .sum()
    }

    pub fn maximum_units_fold(mut box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        box_types.sort_by_key(|x| -x[1]);
        box_types
            .into_iter()
            .fold((truck_size, 0), |(s, acc), p| {
                ((s - p[0]).max(0), acc + p[1] * p[0].min(s))
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            8,
            Solution::maximum_units(vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            91,
            Solution::maximum_units_scan(vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]], 10)
        );
    }
}
