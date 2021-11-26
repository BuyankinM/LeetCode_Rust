// 824. Goat Latin
// https://leetcode.com/problems/goat-latin/

use crate::Solution;

impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut v = Vec::new();
        for (n, word) in sentence.split_ascii_whitespace().enumerate() {
            let trans_word = match word.starts_with(&vowels[..]) {
                true => word.to_string(),
                false => word[1..].to_string() + &word[..1],
            } + "ma";
            v.push(trans_word + &"a".repeat(n + 1))
        }
        v.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "Imaa peaksmaaa oatGmaaaa atinLmaaaaa".to_string(),
            Solution::to_goat_latin("I speak Goat Latin".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa".to_string(),
            Solution::to_goat_latin("The quick brown fox jumped over the lazy dog".to_string())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "Eachmaa ordwmaaa onsistscmaaaa ofmaaaaa owercaselmaaaaaa andmaaaaaaa uppercasemaaaaaaaa etterslmaaaaaaaaa onlymaaaaaaaaaa".to_string(),
            Solution::to_goat_latin("Each word consists of lowercase and uppercase letters only".to_string())
        );
    }
}
