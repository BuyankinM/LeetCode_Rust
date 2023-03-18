// 1472. Design Browser History
// https://leetcode.com/problems/design-browser-history/

use crate::Solution;

struct BrowserHistory {
    history: Vec<String>,
    pos: usize,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self {
            history: vec![homepage],
            pos: 0,
        }
    }

    fn visit(&mut self, url: String) {
        self.pos += 1;
        self.history.truncate(self.pos);
        self.history.push(url);
    }

    fn back(&mut self, steps: i32) -> String {
        self.pos = self.pos.saturating_sub(steps as usize);
        self.history[self.pos].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        self.pos = (self.history.len() - 1).min(self.pos + steps as usize);
        self.history[self.pos].clone()
    }
}
