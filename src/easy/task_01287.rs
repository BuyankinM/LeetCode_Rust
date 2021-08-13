// 1287. Element Appearing More Than 25% In Sorted Array
// https://leetcode.com/problems/element-appearing-more-than-25-in-sorted-array/

use crate::Solution;

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        if arr.len() < 4 {
            return arr[0];
        }

        let q = arr.len() / 4 + 1;
        let (mut counter, mut prev_val) = (0, -1);

        for &a in arr.iter() {
            match prev_val == a {
                true => counter += 1,
                false => {
                    counter = 1;
                    prev_val = a;
                }
            }
            if counter >= q {
                break;
            }
        }
        prev_val
    }

    pub fn find_special_integer_window(arr: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, arr.len() / 4);
        while j < arr.len() {
            if arr[i] == arr[j] {
                return arr[i];
            }
            i += 1;
            j += 1;
        }
        unreachable!();
    }

    pub fn find_special_integer_binsearch(arr: Vec<i32>) -> i32 {
        fn bin_search(nums: &[i32], target: i32, mut low: i32, mut high: i32, first: bool) -> i32 {
            while low <= high {
                let mid = low + (high - low) / 2;
                match (first && nums[mid as usize] < target)
                    || (!first && nums[mid as usize] <= target)
                {
                    true => low = mid + 1,
                    false => high = mid - 1,
                };
            }
            match first {
                true => (high + 1),
                false => (low - 1),
            }
        }

        let l = arr.len();
        if l < 4 {
            return arr[0];
        }

        for i in 1..4 {
            let ind = i * l / 4; // quarter border
            let start = ((i - 1) * l / 4) as i32; // previous quarter border
            let end = ((i + 1) * l / 4 - 1) as i32; // next quarter border
            let min_ind = bin_search(&arr, arr[ind], start, ind as i32, true); // search for the first occurrence
            let max_ind = bin_search(&arr, arr[ind], ind as i32, end, false); // search for the last occurrence
            if (max_ind - min_ind + 1) > l as i32 / 4 {
                return arr[ind];
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            6,
            Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            6,
            Solution::find_special_integer_window(vec![1, 2, 2, 6, 6, 6, 6, 7, 10])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            1,
            Solution::find_special_integer_binsearch(vec![1, 1, 1, 2, 2, 6, 6, 7, 10])
        );
    }
}
