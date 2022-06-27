// 1689. Partitioning Into Minimum Number Of Deci-Binary Numbers
// https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/

use crate::Solution;

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut res = 0;
        for b in n.bytes() {
            if b > res {
                res = b;
                if res == b'9' {
                    break;
                }
            }
        }
        (res - b'0') as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::min_partitions("32".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(8, Solution::min_partitions("82734".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            9,
            Solution::min_partitions("27346209830709182346".to_string())
        );
    }
}
