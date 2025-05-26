#[macro_use]
extern crate queues;

use queues::*;

struct ExpirationQueue {
    q: Queue<String>,
}

impl ExpirationQueue {

    fn push(&mut self, item: String) {
        self.q.add(item).expect("An unexpected error occurred: Couldn't push item to empty Expiration Queue");
        
        if self.q.size() > 10 {
            self.q.remove().expect("An unexpected error occurred: Tried to remove item from empty Expiration Queue");
        } else {
            0;
        }
    }
}

fn main() {

    let mut expiration_queue= ExpirationQueue {
        q: queue![],
    };
    
    expiration_queue.push("hello".to_string());

}