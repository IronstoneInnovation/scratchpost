#[macro_use]
extern crate queues;

use std::collections::HashMap; 
use queues::*;

const MAX_ITEMS: usize = 10;

struct ExpirationQueue {
    q: Queue<String>,
}

impl ExpirationQueue {
    fn push(&mut self, key: String) -> Option<String> {
        self.q.add(key).expect("An unexpected error occurred: Couldn't push key to empty Expiration Queue");
        
        if self.q.size() > MAX_ITEMS {
            let expired_key = self.q.remove().expect("An unexpected error occurred: Tried to remove key from empty Expiration Queue");
            Some(expired_key)
        } else {
            None
        }
    }
}

struct SimpleCache {
    expiration_queue: ExpirationQueue,
    items: HashMap<String, String>,
}

impl SimpleCache {
    fn get(&mut self, key: String, value: String) -> Result<> {


    }
}

fn main() {

    let mut expiration_queue= ExpirationQueue {
        q: queue![],
    };
    
    let simple_cache = SimpleCache {
        expiration_queue: expiration_queue,
        items: HashMap::new(),
    };

    //expiration_queue.push("hello".to_string());

}