// 443. String Compression
// https://leetcode.com/problems/string-compression/description/

use crate::Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        fn correct(chars: &mut [char], count: usize, pos: usize, prev_c: char) -> usize {
            let l = match count <= 1 {
                true => 1,
                false => 2 + count.ilog10() as usize,
            };
            chars[pos] = prev_c;
            if count > 1 {
                chars[pos + 1..]
                    .iter_mut()
                    .zip(count.to_string().chars())
                    .for_each(|(c, d)| *c = d);
            }
            l
        }

        let mut count = 1;
        let mut pos = 0;
        let mut prev_c = ' ';

        for i in 0..chars.len() {
            let c = chars[i];
            match prev_c == c {
                true => count += 1,
                false if prev_c != ' ' => {
                    pos += correct(chars, count, pos, prev_c);
                    count = 1;
                }
                _ => (),
            };
            prev_c = c;
        }
        pos += correct(chars, count, pos, prev_c);
        pos as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            6,
            Solution::compress(&mut vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'])
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::compress(&mut vec!['a']))
    }

    #[test]
    fn test_3() {
        assert_eq!(
            10,
            Solution::compress(&mut vec!['G', 't', 'W', 'Y', 'v', '&', ':', 'd', '#', 'k'])
        )
    }
}
