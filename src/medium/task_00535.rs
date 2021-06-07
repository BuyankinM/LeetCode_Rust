// 535. Encode and Decode TinyURL
// https://leetcode.com/problems/encode-and-decode-tinyurl/

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::cell::RefCell;
use rand::{distributions::Alphanumeric, Rng};

#[derive(Default)]
struct Codec {
    to_long_url: RefCell<HashMap<String, String>>,
    to_short_url: RefCell<HashMap<String, String>>
}

impl Codec {
    fn new() -> Self {
        Default::default()
    }
	
    // Encodes a URL to a shortened URL.
    fn encode(&self, long_url: String) -> String {

        let mut to_long_url = self.to_long_url.borrow_mut();
        let mut to_short_url = self.to_short_url.borrow_mut();

        match to_short_url.entry(long_url.clone()) {
            Entry::Occupied(short_val) => short_val.get().clone(),
            Entry::Vacant(short_val) => loop {
                let short_url: String = "https://tinyurl.com/"
                    .chars()
                    .chain(rand::thread_rng().sample_iter(&Alphanumeric).take(7).map(char::from))
                    .collect();

                if let Entry::Vacant(long_val) = to_long_url.entry(short_url.clone()) {
                    long_val.insert(long_url);
                    break short_val.insert(short_url).clone();
                }
            }
        }
    }
	
    // Decodes a shortened URL to its original URL.
    fn decode(&self, short_url: String) -> String {

        self.to_long_url.borrow().get(&short_url).unwrap().clone()

    }
}
