use std::net::UdpSocket;
use std::collections::vec_deque::VecDeque;
use std::collections::binary_heap::BinaryHeap;
use std::string::String;
use packet;
use queue_controller;

pub struct Actor {
    sock: UdpSocket,
    addr: String,
    curr_seq_num: u32,
    q_ctrller: queue_controller::QueueController,
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

    pub fn prepare_msg(&mut self, msg: &str) -> () {
        // TODO packet chunking
        let packet = packet::Packet{flag: packet::Flag::PSH,
                                    seq_num: self.curr_seq_num,
                                    msg: msg.as_bytes().to_vec()};
        self.q_ctrller.enq_out_msg(packet);
        self.curr_seq_num += 1;
    }

    pub fn send_msg(&mut self, addr: &str) -> () {
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

