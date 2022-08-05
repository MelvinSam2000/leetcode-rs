#[derive(PartialEq, Eq)]
enum State {
    Normal,
    Full,
    Empty,
}

pub struct MyCircularQueue {
    buffer: Vec<i32>,
    front: usize,  // points to free spot
    bottom: usize, // points to cur element to peek
    cap: usize,
    state: State,
}

impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        Self {
            buffer: vec![0; k as usize],
            front: 0,
            bottom: 0,
            cap: k as usize,
            state: State::Empty,
        }
    }

    // O(1)
    pub fn en_queue(&mut self, value: i32) -> bool {
        match self.state {
            State::Normal => {
                self.buffer[self.front] = value;
                self.front = (self.front + 1) % self.cap;
                if self.front == self.bottom {
                    self.state = State::Full;
                }
                true
            }
            State::Full => false,
            State::Empty => {
                self.buffer[self.front] = value;
                self.front = (self.front + 1) % self.cap;
                self.state = if self.front == self.bottom {
                    State::Full
                } else {
                    State::Normal
                };
                true
            }
        }
    }

    // O(1)
    pub fn de_queue(&mut self) -> bool {
        match self.state {
            State::Normal => {
                self.bottom = (self.bottom + 1) % self.cap;
                if self.bottom == self.front {
                    self.state = State::Empty;
                }
                true
            }
            State::Full => {
                self.bottom = (self.bottom + 1) % self.cap;
                self.state = if self.front == self.bottom {
                    State::Empty
                } else {
                    State::Normal
                };
                true
            }
            State::Empty => false,
        }
    }

    // O(1)
    pub fn front(&self) -> i32 {
        if self.state == State::Empty {
            -1
        } else {
            self.buffer[self.bottom]
        }
    }

    // O(1)
    pub fn rear(&self) -> i32 {
        if self.state == State::Empty {
            -1
        } else {
            let index = if self.front == 0 {
                self.cap - 1
            } else {
                self.front - 1
            };
            self.buffer[index]
        }
    }

    // O(1)
    pub fn is_empty(&self) -> bool {
        self.state == State::Empty
    }

    // O(1)
    pub fn is_full(&self) -> bool {
        self.state == State::Full
    }
}
