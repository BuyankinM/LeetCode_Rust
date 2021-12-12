// 2103. Rings and Rods
// https://leetcode.com/problems/rings-and-rods/

use crate::Solution;

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut rods = [0; 10];
        rings.as_bytes().chunks(2).for_each(|pair| {
            rods[(pair[1] - b'0') as usize] |= match pair[0] {
                b'R' => 0b001,
                b'G' => 0b010,
                _ => 0b100,
            };
        });
        rods.iter().filter(|&&x| x == 0b111).count() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::count_points("B0B6G0R6R0R6G9".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::count_points("B0R0G0R9R0B0G0".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::count_points("G4".to_string()));
    }
}
