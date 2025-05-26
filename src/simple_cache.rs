extern crate queues;

use std::collections::HashMap; 
use queues::*;

// TODO: Get MAX_ITEMS from envvars
const MAX_ITEMS: usize = 10;

pub struct ExpirationQueue {
    pub q: Queue<String>,
}

impl ExpirationQueue {
    fn push(&mut self, key: String) -> Option<String> {
        self.q.add(key).expect("Couldn't push key to empty Expiration Queue");
        
        if self.q.size() > MAX_ITEMS {
            let expired_key = self.q.remove().expect("Tried to remove key from empty Expiration Queue");
            Some(expired_key)
        } else {
            None
        }
    }
}

pub struct SimpleCache {
    pub expiration_queue: ExpirationQueue,
    pub items: HashMap<String, String>,
}

impl SimpleCache {

    pub fn get(&mut self, key: String) -> String {
        match self.items.get(&key) {
            Some(value) => value.to_string(),
            None => "".to_string()
        }
    }

    pub fn push(&mut self, key: String, value: String) {
        let key_for_expiration_queue = key.clone();

        self.items.insert(key, value);

        match self.expiration_queue.push(key_for_expiration_queue) {
            Some(k) => {
                self.items.remove(&k).expect("Could not removed an expired key from the Expiration Queue");
            },
            None => return
        }
    }

}

pub fn new() -> SimpleCache {
    let expiration_queue= ExpirationQueue {
        q: queue![],
    };
    
    SimpleCache {
        expiration_queue: expiration_queue,
        items: HashMap::new(),
    }
}