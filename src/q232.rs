/*
    232 - Queue using 2 stacks
*/
#[derive(Default)]
pub struct MyQueue {
    s1: Vec<i32>,
    s2: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        Self::default()
    }

    // O(1)
    pub fn push(&mut self, x: i32) {
        self.s1.push(x);
    }

    // Average: O(1), Worst: O(n)
    pub fn pop(&mut self) -> i32 {
        match self.s2.pop() {
            Some(x) => x,
            None => {
                while let Some(x) = self.s1.pop() {
                    self.s2.push(x);
                }
                self.s2.pop().unwrap()
            }
        }
    }

    // Average: O(1), Worst: O(n)
    pub fn peek(&mut self) -> i32 {
        match self.s2.last() {
            Some(&x) => x,
            None => {
                while let Some(x) = self.s1.pop() {
                    self.s2.push(x);
                }
                *self.s2.last().unwrap()
            }
        }
    }

    // O(1)
    pub fn empty(&self) -> bool {
        self.s1.is_empty() && self.s2.is_empty()
    }
}
