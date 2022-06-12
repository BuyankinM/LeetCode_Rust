// 1695. Maximum Erasure Value
// https://leetcode.com/problems/maximum-erasure-value/

use crate::Solution;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut positions_sums = [(-1, 0); 10_001];
        let (mut max_sum, mut cur_sum, mut prev_sum) = (0, 0, 0);
        let mut start = 0;

        for (&num, i) in nums.iter().zip(0..) {
            let pos_sum = &mut positions_sums[num as usize];

            if pos_sum.0 >= start {
                max_sum = max_sum.max(cur_sum - prev_sum);
                start = pos_sum.0 + 1;
                prev_sum = pos_sum.1;
            }

            cur_sum += num;
            *pos_sum = (i, cur_sum);
        }
        max_sum.max(cur_sum - prev_sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(17, Solution::maximum_unique_subarray(vec![4, 2, 4, 5, 6]));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            8,
            Solution::maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(10000, Solution::maximum_unique_subarray(vec![10000]));
    }

    #[test]
    fn test_4() {
        assert_eq!(
            10001,
            Solution::maximum_unique_subarray(vec![10000, 1, 10000, 1, 1, 1, 1, 1, 1])
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            30934,
            Solution::maximum_unique_subarray(vec![
                449, 154, 934, 526, 429, 732, 784, 909, 884, 805, 635, 660, 742, 209, 742, 272,
                669, 449, 766, 904, 698, 434, 280, 332, 876, 200, 333, 464, 12, 437, 269, 355, 622,
                903, 262, 691, 768, 894, 929, 628, 867, 844, 208, 384, 644, 511, 908, 792, 56, 872,
                275, 598, 633, 502, 894, 999, 788, 394, 309, 950, 159, 178, 403, 110, 670, 234,
                119, 953, 267, 634, 330, 410, 137, 805, 317, 470, 563, 900, 545, 308, 531, 428,
                526, 593, 638, 651, 320, 874, 810, 666, 180, 521, 452, 131, 201, 915, 502, 765, 17,
                577, 821, 731, 925, 953, 111, 305, 705, 162, 994, 425, 17, 140, 700, 475, 772, 385,
                922, 159, 840, 367, 276, 635, 696, 70, 744, 804, 63, 448, 435, 242, 507, 764, 373,
                314, 140, 825, 34, 383, 151, 602, 745
            ])
        );
    }
}
