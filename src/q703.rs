use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct KthLargest {
    pq: BinaryHeap<Reverse<i32>>,
    cap: usize,
}

/*
    703 - Kth Largest Element in a stream
*/
impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut out = Self {
            pq: BinaryHeap::new(),
            cap: k as usize,
        };
        for elem in nums {
            out.pq.push(Reverse(elem));
            if out.pq.len() > out.cap {
                out.pq.pop();
            }
        }
        out
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.pq.push(Reverse(val));
        if self.pq.len() > self.cap {
            self.pq.pop();
        }
        self.pq.peek().cloned().unwrap().0
    }
}
