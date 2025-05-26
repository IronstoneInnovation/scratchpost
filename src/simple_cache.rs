//! # Simple Crate
//! 
//! This crate provides the basic cache functionality for scratchpost.
//! 
extern crate queues;

use std::env;
use std::collections::HashMap; 
use queues::*;

/// This defines the default cache size if the SCRATCHPOST_MAX_ITEMS envvar is not set.
pub const DEFAULT_MAX_ITEMS: usize = 10;

/// Return the maximum number of items the cache can store.  This will be the value of
/// the SCRATCHPOST_MAX_ITEMS envvar if it is set correctly, otherwise we'll use DEFAULT_MAX_ITEMS.
pub fn get_max_items() -> usize {
    
    match env::var("SCRATCHPOST_MAX_ITEMS") {
        Ok(str_value) => {
            match str_value.parse::<usize>() {
                Ok(value) => value,
                Err(_) => DEFAULT_MAX_ITEMS
            }
        },
        Err(_) => DEFAULT_MAX_ITEMS
    }
}

/// This queue is for keeping track of the oldest key so we know which item to
/// remove from the cache to make space for a new item when the cache is full.
pub struct ExpirationQueue {
    q: Queue<String>,
}

impl ExpirationQueue {
    /// Enqueue a new key.  If the queue is already full, remove the oldest
    /// key from the queue and return it, otherwise return None.
    fn push(&mut self, key: String) -> Option<String> {
        
        self.q.add(key).expect("Couldn't push key to empty Expiration Queue");
        
        let max_items = get_max_items();

        if self.q.size() > max_items {
            let expired_key = self.q.remove().expect("Tried to remove key from empty Expiration Queue");
            Some(expired_key)
        } else {
            None
        }
    }
}

/// Contains the cache itself (items) and the Expiration Queue used to track
/// the oldest items which will need to be removed to make space for the new
/// items.
pub struct SimpleCache {
    expiration_queue: ExpirationQueue,
    items: HashMap<String, String>,
}

impl SimpleCache {

    /// Return the value of a item.  If the item does not exist, an empty string ""
    /// will be returned instead.
    pub fn get(&mut self, key: String) -> String {
        match self.items.get(&key) {
            Some(value) => value.to_string(),
            None => "".to_string()
        }
    }

    /// Push a new item in to the cache.
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

    /// Returns the number of items in the cache (for testing only).
    pub fn size(&mut self) -> usize {
        
        self.items.len()
    }

}

/// Returns a new SimpleCache.
pub fn new() -> SimpleCache {
    let expiration_queue= ExpirationQueue {
        q: queue![],
    };
    
    SimpleCache {
        expiration_queue: expiration_queue,
        items: HashMap::new(),
    }
}