use std::net::UdpSocket;
use std::collections::vec_deque::VecDeque;
use std::collections::binary_heap::BinaryHeap;
use std::string::String;
use atomic_mq;
use packet;

pub struct Actor {
    sock: UdpSocket,
    addr: String,
    curr_seq_num: u32,
    in_mq: atomic_mq::AtomicMq<BinaryHeap<packet::Packet>, packet::Packet>,
    out_mq: atomic_mq::AtomicMq<VecDeque<packet::Packet>, packet::Packet>,
}

impl Actor {

    pub fn new(ip: &str, port: &str) -> Actor {
        Actor {
            addr: ip.to_string() + ":" + port,
            curr_seq_num: 1,
            in_mq: atomic_mq::AtomicMq::new(BinaryHeap::new()),
            out_mq: atomic_mq::AtomicMq::new(VecDeque::new()),
            sock: UdpSocket::bind(&(ip.to_string() + ":" + port) as &str)
                            .ok()
                            .expect(&format!("Unable to bind to IP {} at port {}!",
                                              ip, port)),
        }
    }

    pub fn send_msg(&mut self, addr: &str) -> () {
        if let Some(m) = self.out_mq.lock_and(VecDeque::pop_front) {
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
            _ => self.in_mq.lock_and_arg(BinaryHeap::push, packet::to_packet(&buf)),
        }
    }

    pub fn enq_out_msg(&mut self, msg: &str) -> () {
        // TODO packet chunking
        let packet = packet::Packet{flag: packet::Flag::PSH,
                                    seq_num: self.curr_seq_num,
                                    msg: msg.as_bytes().to_vec()};
        self.out_mq.lock_and_arg(VecDeque::push_back, packet);
        self.curr_seq_num += 1;
    }

    pub fn dq_inc_msg(&mut self) -> String {
        if let Some(p) = self.in_mq.lock_and(BinaryHeap::pop) {
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
    use packet;
    use std::collections::VecDeque;

    #[test]
    fn test_enq_out_msg() {
        let mut actor = Actor::new("127.0.0.1", "8000");
        actor.enq_out_msg("test1");
        actor.enq_out_msg("test2");
        assert_eq!(actor.curr_seq_num, 3);
        assert_eq!(actor.out_mq.lock_and(VecDeque::len), 2);
        for (idx, elt) in actor.out_mq.iter().enumerate() {
            assert_eq!(elt.flag, packet::Flag::PSH);
            assert_eq!(elt.seq_num, (idx + 1) as u32);
            assert_eq!(elt.msg, format!("test{}", idx + 1).as_bytes().to_vec());
        }
    }

    /*#[test]
    fn test_dq_in_msg() {
        let mut actor = Actor::new("127.0.0.1", "8000");
        for idx in [3,7,5,1].into_iter() {
            actor.in_mq
                 .try_lock()
                 .unwrap()
                 .push(packet::Packet{flag: packet::Flag::PSH,
                                      seq_num: *idx,
                                      msg: format!("test{}", *idx)
                                           .as_bytes()
                                           .to_vec()});
        }
        let mut i = 1;
        while let Some(elt) = actor.in_mq
                                   .try_lock()
                                   .unwrap()
                                   .pop() {
            assert_eq!(elt.seq_num, i);
            assert_eq!(elt.msg, format!("test{}", i).as_bytes().to_vec());
            i += 2;
        }
    }

    #[test]
    fn test_send_recv_simple() {
        let mut sender = Actor::new("127.0.0.1", "8001");
        let mut receiver = Actor::new("127.0.0.1", "8000");
        let test_msg = "test";
        sender.enq_out_msg(test_msg);
        sender.send_msg("127.0.0.1:8000");
        receiver.recv_msg();
        let msg = receiver.dq_inc_msg();
        assert_eq!(msg, test_msg);
    }*/
}
