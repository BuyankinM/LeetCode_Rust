// 1108. Defanging an IP Address
// https://leetcode.com/problems/defanging-an-ip-address/

use crate::Solution;

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace(".", "[.]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "1[.]1[.]1[.]1".to_owned(),
            Solution::defang_i_paddr("1.1.1.1".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "255[.]100[.]50[.]0".to_owned(),
            Solution::defang_i_paddr("255.100.50.0".to_owned())
        );
    }
}
