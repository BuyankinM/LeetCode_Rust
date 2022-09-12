// 948. Bag of Tokens
// https://leetcode.com/problems/bag-of-tokens/

use crate::Solution;

impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        match tokens.len() {
            0 => 0,
            1 => (power >= tokens[0]) as i32,
            l => {
                tokens.sort_unstable();

                let (mut i, mut j) = (0, l - 1);
                let mut res = 0;

                while i <= j {
                    match power >= tokens[i] {
                        true => {
                            power -= tokens[i];
                            res += 1;
                            i += 1;
                        }
                        false if (i < j) && (res > 0) => {
                            power += tokens[j];
                            res -= 1;
                            j -= 1;
                        }
                        _ => break,
                    }
                }
                res
            }
        }
    }

    // https://leetcode.com/problems/bag-of-tokens/discuss/2565393/Rust-iterator-solution
    pub fn bag_of_tokens_score_iter(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        tokens.sort_unstable();
        let mut tokens = tokens.into_iter();
        let mut score = 0;

        while let Some(p) = tokens.next() {
            if p > power {
                if score > 0 {
                    power += tokens.next_back().unwrap_or(p) - p;
                } else {
                    return score;
                }
            } else {
                power -= p;
                score += 1;
            }
        }

        score
    }

    // https://leetcode.com/problems/bag-of-tokens/discuss/2565603/Rust-100-runtime-100-memory-easy-idiomatic
    pub fn bag_of_tokens_score_vecdeque(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        use std::collections::VecDeque;

        tokens.sort_unstable();
        let mut tokens: VecDeque<_> = tokens.into_iter().collect();

        let mut score = 0;
        while !tokens.is_empty() {
            if power >= *tokens.front().unwrap() {
                power -= tokens.pop_front().unwrap();
                score += 1;
            } else if tokens.len() > 1
                && score > 0
                && power + *tokens.back().unwrap() >= *tokens.front().unwrap()
            {
                power += tokens.pop_back().unwrap();
                score -= 1;
            } else {
                break;
            }
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::bag_of_tokens_score(vec![100], 50), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::bag_of_tokens_score(vec![100, 200], 150), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::bag_of_tokens_score(vec![100, 200, 300, 400], 200),
            2
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::bag_of_tokens_score(vec![26], 51), 1);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::bag_of_tokens_score(vec![71, 55, 82], 54), 0);
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::bag_of_tokens_score(vec![81, 91, 31], 73), 1);
    }
}
