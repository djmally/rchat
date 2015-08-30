mod actor;
mod packet;
mod message_processor;
mod queue_controller;
use actor::Actor;

extern crate bincode;
extern crate rustc_serialize;


fn main() {
    let mut sender = Actor::new("127.0.0.1", "8001");
    let mut receiver = Actor::new("127.0.0.1", "8000");
    let test_msg = message_processor::read_stdin();
    sender.prepare_msg(&test_msg);
    sender.send_msg("127.0.0.1:8000");
    receiver.recv_msg();
    println!("{}", receiver.read_msg());
}
