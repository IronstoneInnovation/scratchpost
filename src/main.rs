#[macro_use]
extern crate queues;

use queues::*;

const MAX_ITEMS: usize = 10;

struct ExpirationQueue {
    q: Queue<String>,
}

impl ExpirationQueue {

    fn push(&mut self, item: String) -> String {
        self.q.add(item).expect("An unexpected error occurred: Couldn't push item to empty Expiration Queue");
        
        if self.q.size() > MAX_ITEMS {
            let expired_item_key = self.q.remove().expect("An unexpected error occurred: Tried to remove item from empty Expiration Queue");
            expired_item_key
        } else {
            "".to_string()
        }
    }

}

fn main() {

    let mut expiration_queue= ExpirationQueue {
        q: queue![],
    };
    
    expiration_queue.push("hello".to_string());

}