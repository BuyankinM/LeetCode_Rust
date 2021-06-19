// 1656. Design an Ordered Stream
// https://leetcode.com/problems/design-an-ordered-stream/

struct OrderedStream {
    ary: Vec<Option<String>>,
    nxt: usize,
}

impl OrderedStream {
    fn new(n: i32) -> Self {
        OrderedStream {
            ary: vec![None; n as usize],
            nxt: 0,
        }
    }

    fn insert(&mut self, key: i32, value: String) -> Vec<String> {
        self.ary[(key - 1) as usize] = Some(value);

        let mut res = Vec::new();
        let mut read = 0;

        for i in self.nxt..self.ary.len() {
            if let Some(s) = self.ary[i].take() {
                res.push(s);
                read += 1;
            } else {
                break;
            }
        }
        self.nxt += read;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut obj = OrderedStream::new(5);

        let ret = obj.insert(3, "ccccc".to_owned());
        let empty: Vec<String> = Vec::new();
        assert_eq!(empty, ret);

        let ret = obj.insert(1, "aaaaa".to_owned());
        assert_eq!(vec!["aaaaa".to_owned()], ret);

        let ret = obj.insert(2, "bbbbb".to_owned());
        assert_eq!(vec!["bbbbb".to_owned(), "ccccc".to_owned()], ret);

        let ret = obj.insert(5, "eeeee".to_owned());
        assert_eq!(empty, ret);

        let ret = obj.insert(4, "ddddd".to_owned());
        assert_eq!(vec!["ddddd".to_owned(), "eeeee".to_owned()], ret);
    }
}
