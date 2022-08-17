// 2375. Construct Smallest Number From DI String
// https://leetcode.com/problems/construct-smallest-number-from-di-string/

use crate::Solution;

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        // compress pattern "IIIDD" => vec![('I', 3), ('D', 2)]
        let mut pat = Vec::new();
        let mut prev_c = ' ';
        for c in pattern.chars() {
            if c != prev_c {
                pat.push((c, 0));
            }
            pat.last_mut().unwrap().1 += 1;
            prev_c = c;
        }

        // we use bit mask and trailing_ones() method to get next "free" minimum index
        let mut mask_count = 0_u16;

        let mut res = Vec::new();
        let mut i = 0;
        for (c, n) in pat {
            // init start position for current group inc/desc
            let inc = c == 'I';
            match inc {
                true => i = mask_count.trailing_ones(), // first free index
                false => i += n, // minimum index to descend down for the current n
            }

            for _ in 0..n {
                res.push((i as u8 + b'1') as char);
                mask_count |= 1 << i; // update mask

                // calc next position
                match inc {
                    true => i = mask_count.trailing_ones(),
                    false => i -= 1,
                }
            }
        }

        // last symbol
        res.push((i as u8 + b'1') as char);

        res.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::smallest_number("IIIDIDDD".to_string()),
            "123549876".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::smallest_number("DDD".to_string()),
            "4321".to_string()
        );
    }
}
