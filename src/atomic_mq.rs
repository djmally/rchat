use std::collections::vec_deque::VecDeque;
use std::collections::binary_heap::BinaryHeap;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

pub trait MessageQueue {}

impl<T> MessageQueue for VecDeque<T> {}
impl<T> MessageQueue for BinaryHeap<T> {}

pub struct AtomicMq<T, U> where T: MessageQueue {
    q: Arc<Mutex<T>>,
    inner: PhantomData<U>,
}

impl<T, U> AtomicMq<T, U> where T: MessageQueue {
    pub fn new(t: T) -> AtomicMq<T, U> {
        AtomicMq {
            q: Arc::new(Mutex::new(t)),
            inner: PhantomData,
        }
    }

    pub fn lock_and<'m, F, V>(&mut self, mut f: F) -> V where F: FnMut(&mut T) -> V {
        let ref mut tmp = *self.q.try_lock().unwrap();
        f(tmp)
    }

    // TODO How do I avoid needing this function?
    pub fn lock_and_arg<'m, F>(&mut self, mut f: F, arg: U) -> () where F: FnMut(&mut T, U) -> () {
        let ref mut tmp = *self.q.try_lock().unwrap();
        f(tmp, arg)
    }
}
