use std::sync::{Arc, Mutex};
use std::net::UdpSocket;
use std::string::String;
use std::thread;
use message_processor;
use packet;
use queue_controller;

pub struct Actor {
    sock: UdpSocket,
    addr: String,
    curr_seq_num: u32,
    q_ctrller: queue_controller::QueueController,
}

pub fn event_loop(actor_mutex: Arc<Mutex<Actor>>) -> () {
    let (sender_clone, receiver_clone) = (actor_mutex.clone(), actor_mutex.clone());
    thread::Builder::new().name("sender".to_string()).spawn(move || {
        loop {
            let ref mut actor: Actor = *sender_clone.lock().unwrap();
            actor.prepare_msg(&message_processor::read_stdin());
            actor.send_msg("127.0.0.1:8000");
        }
    }).ok().expect("Unable to spawn thread sender")
      .join().ok().expect("Unable to join thread sender");
    thread::Builder::new().name("receiver".to_string()).spawn(move || {
        loop {
            let ref mut actor: Actor = *receiver_clone.lock().unwrap();
            actor.recv_msg();
            println!("{}", actor.read_msg());
        }
    }).ok().expect("Unable to spawn thread receiver")
      .join().ok().expect("Unable to join thread receiver");
}


impl Actor {

    pub fn new(ip: &str, port: &str) -> Actor {
        Actor {
            addr: ip.to_string() + ":" + port,
            curr_seq_num: 1,
            q_ctrller: queue_controller::QueueController::new(),
            sock: UdpSocket::bind(&(ip.to_string() + ":" + port) as &str)
                             .ok()
                             .expect(&format!("Unable to bind to IP {} at port {}!",
                                              ip, port)),
        }
    }

    fn prepare_msg(&mut self, msg: &str) -> () {
        // TODO packet chunking
        let packet = packet::Packet{flag: packet::Flag::PSH,
                                    seq_num: self.curr_seq_num,
                                    msg: msg.as_bytes().to_vec()};
        self.q_ctrller.enq_out_msg(packet);
        self.curr_seq_num += 1;
    }

    fn send_msg(&mut self, addr: &str) -> () {
        if let Some(m) = self.q_ctrller.deq_out_msg() {
            let bytes = self.sock.send_to(&m.as_bytes(), addr)
                                 .unwrap_or(0);
            if bytes == 0 { println!("Unable to send to address {}!", addr) }
        }
    }

    pub fn recv_msg(&mut self) -> () {
        let mut buf = [0;1024];
        let recv_result = self.sock.recv_from(&mut buf);
        match recv_result {
            Ok((0, _)) | Err(..) => println!("Unable to receive message from address {}!",
                                             self.addr),
            _ => self.q_ctrller.enq_in_msg(packet::to_packet(&buf)),
        }
    }

    pub fn read_msg(&mut self) -> String {
        if let Some(p) = self.q_ctrller.deq_in_msg() {
            if let Ok(msg) = String::from_utf8(p.msg) {
                return msg;
            }
        }
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_recv_simple() {
        let mut sender = Actor::new("127.0.0.1", "8001");
        let mut receiver = Actor::new("127.0.0.1", "8000");
        let test_msg = "test";
        sender.prepare_msg(test_msg);
        sender.send_msg("127.0.0.1:8000");
        receiver.recv_msg();
        let msg = receiver.read_msg();
        assert_eq!(msg, test_msg);
    }
}
