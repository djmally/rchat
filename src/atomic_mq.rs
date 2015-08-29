use std::sync::{Arc, Mutex};

pub trait MessageQueue{}

pub struct AtomicMq<T> where T: MessageQueue {
    q: Arc<Mutex<T>>,
}

impl<T> AtomicMq<T> where T: MessageQueue {
    fn lock_and<'m, F, U>(self, f: F) -> U where F: FnOnce(&T) -> U {
        let ref mut tmp = *self.q.try_lock().unwrap();
        f(tmp)
    }
}
