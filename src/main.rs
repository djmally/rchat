mod actor;
mod packet;
mod message_processor;
mod atomic_mq;
use actor::Actor;

extern crate bincode;
extern crate rustc_serialize;


fn main() {
    let mut listener = Actor::new("127.0.0.1", "8000");
    let mut sender = Actor::new("127.0.0.1", "8001");
    let msg = message_processor::read_message();
    sender.enq_out_msg(&msg);
    sender.send_msg("127.0.0.1:8000");
    listener.recv_msg();
    listener.dq_inc_msg();
}
