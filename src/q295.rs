/*
    295 - Find median from data stream
*/

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Default)]
pub struct MedianFinder {
    low_half: BinaryHeap<i32>,           // max heap
    high_half: BinaryHeap<Reverse<i32>>, // min heap
    len: usize,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self::default()
    }

    // O(logn)
    pub fn add_num(&mut self, num: i32) {
        if self.len % 2 == 0 {
            if let Some(&Reverse(hi_elem)) = self.high_half.peek() {
                if num < hi_elem {
                    self.low_half.push(num);
                } else {
                    self.high_half.push(Reverse(num));
                    let elem = self.high_half.pop().unwrap().0;
                    self.low_half.push(elem);
                }
            } else {
                self.low_half.push(num);
            }
        } else {
            self.low_half.push(num);
            let elem = self.low_half.pop().unwrap();
            self.high_half.push(Reverse(elem));
        }
        self.len += 1;
    }

    // O(1)
    pub fn find_median(&self) -> f64 {
        if self.len % 2 == 0 {
            let x1 = *self.low_half.peek().unwrap() as f64;
            let x2 = self.high_half.peek().unwrap().0 as f64;
            (x1 + x2) / 2.0
        } else {
            *self.low_half.peek().unwrap() as f64
        }
    }
}
