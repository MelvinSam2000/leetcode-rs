/*
    155 - Min stack
    Time: All operations are O(1)
*/
#[derive(Default)]
pub struct MinStack {
    stack: Vec<i32>,
    mins: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.mins.is_empty() || val <= self.get_min() {
            self.mins.push(val);
        }
    }

    pub fn pop(&mut self) {
        let val = self.stack.pop().unwrap();
        if val <= self.get_min() {
            self.mins.pop();
        }
    }

    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.mins.last().unwrap()
    }
}
