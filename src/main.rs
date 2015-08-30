mod actor;
mod packet;
mod message_processor;
mod queue_controller;

use actor::Actor;
use std::sync::{Arc, Mutex};

extern crate bincode;
extern crate rustc_serialize;


fn main() {
    let sender = Arc::new(Mutex::new(Actor::new("127.0.0.1", "8001")));
    let mut receiver = Actor::new("127.0.0.1", "8000");
    actor::event_loop(sender);
    receiver.recv_msg();
    println!("{}", receiver.read_msg());
}
