// 929. Unique Email Addresses
// https://leetcode.com/problems/unique-email-addresses/

use crate::Solution;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        use std::collections::HashSet;
        emails
            .iter()
            .map(|s| {
                let (mut plus, mut domain) = (false, false);
                s.chars()
                    .filter(|&c| match c {
                        _ if domain => true,
                        '+' => {
                            plus = true;
                            false
                        }
                        '@' => {
                            domain = true;
                            true
                        }
                        _ if plus => false,
                        '.' => false,
                        _ => true,
                    })
                    .collect::<String>()
            })
            .collect::<HashSet<_>>()
            .len() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::num_unique_emails(vec![
                "test.email+alex@leetcode.com".to_string(),
                "test.e.mail+bob.cathy@leetcode.com".to_string(),
                "testemail+david@lee.tcode.com".to_string()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::num_unique_emails(vec![
                "a@leetcode.com".to_string(),
                "b@leetcode.com".to_string(),
                "c@leetcode.com".to_string()
            ])
        );
    }
}
