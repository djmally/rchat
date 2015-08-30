use std::collections::vec_deque::VecDeque;
use std::collections::binary_heap::BinaryHeap;
use std::sync::{Arc, Mutex};
use packet;

pub struct QueueController {
    in_mq: Arc<Mutex<BinaryHeap<packet::Packet>>>,
    out_mq: Arc<Mutex<VecDeque<packet::Packet>>>,
}

impl QueueController {
    pub fn new() -> QueueController {
        QueueController {
            in_mq: Arc::new(Mutex::new(BinaryHeap::new())),
            out_mq: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn enq_in_msg(&mut self, packet: packet::Packet) -> () {
        self.in_mq.lock().unwrap().push(packet);
    }

    pub fn deq_in_msg(&mut self) -> Option<packet::Packet> {
        self.in_mq.lock().unwrap().pop()
    }

    pub fn enq_out_msg(&mut self, packet: packet::Packet) -> () {
        self.out_mq.lock().unwrap().push_back(packet);
    }

    pub fn deq_out_msg(&mut self) -> Option<packet::Packet> {
        self.out_mq.lock().unwrap().pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use packet;

    #[test]
    fn test_out_mq() {
        let mut q_ctrller = QueueController::new();
        for i in (0..2) {
            q_ctrller.enq_out_msg(packet::Packet{flag: packet::Flag::PSH,
                                  seq_num: i,
                                  msg: format!("test{}", i)
                                       .as_bytes()
                                       .to_vec()});
        }
        assert_eq!(q_ctrller.out_mq.lock().unwrap().len(), 2);
        for (idx, elt) in q_ctrller.out_mq.lock().unwrap().iter().enumerate() {
            assert_eq!(elt.flag, packet::Flag::PSH);
            assert_eq!(elt.seq_num, idx as u32);
            assert_eq!(elt.msg, format!("test{}", idx).as_bytes().to_vec());
        }
    }

    #[test]
    fn test_in_mq() {
        let mut q_ctrller = QueueController::new();
        for idx in [3,7,5,1].into_iter() {
            q_ctrller.enq_in_msg(packet::Packet{flag: packet::Flag::PSH,
                                 seq_num: *idx,
                                 msg: format!("test{}", *idx)
                                      .as_bytes()
                                      .to_vec()});
        }
        let mut i = 1;
        while let Some(elt) = q_ctrller.deq_in_msg() {
            assert_eq!(elt.seq_num, i);
            assert_eq!(elt.msg, format!("test{}", i).as_bytes().to_vec());
            i += 2;
        }
    }

}
