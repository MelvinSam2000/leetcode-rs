/*
    146 - LRU Cache
    Note:
    - Pop and push are both O(1)
    - Implemented using a hashmap and doubly linked list.
    - Requires unsafe (only sane way to write a doubly linked list)
*/
use std::collections::HashMap;
use std::ptr;

pub struct LRUCache {
    map: HashMap<i32, *mut Node>,
    pub len: i32,
    capacity: i32,
    mru: *mut Node,
    lru: *mut Node,
}

struct Node {
    key: i32,
    value: i32,
    next: *mut Node,
    prev: *mut Node,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let mru = Box::into_raw(Box::new(Node {
            key: 0,
            value: 0,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        }));
        let lru = Box::into_raw(Box::new(Node {
            key: 0,
            value: 0,
            next: mru,
            prev: ptr::null_mut(),
        }));
        unsafe { &mut *mru }.prev = lru;

        Self {
            map: HashMap::new(),
            len: 0,
            capacity,
            mru,
            lru,
        }
    }

    // O(1)
    pub fn get(&mut self, key: i32) -> i32 {
        self.map
            .get(&key)
            .cloned()
            .map(|node| {
                if node != self.mru {
                    self.remove_at(node);
                    self.push_mru(node);
                }
                unsafe { &*node }.value
            })
            .unwrap_or(-1)
    }

    // O(1)
    pub fn put(&mut self, key: i32, value: i32) {
        match self.map.get(&key).cloned() {
            Some(node) => {
                unsafe { &mut *node }.value = value;
                if node != self.mru {
                    self.remove_at(node);
                    self.push_mru(node);
                }
            }
            None => {
                let node = Box::into_raw(Box::new(Node {
                    key,
                    value,
                    next: ptr::null_mut(),
                    prev: ptr::null_mut(),
                }));
                self.push_mru(node);
                self.map.insert(key, node);
                self.len += 1;
                if self.len > self.capacity {
                    if let Some(node) = self.pop_lru() {
                        self.map.remove(&unsafe { &*node }.key);
                        self.len -= 1;
                        let _ = unsafe { Box::from_raw(node) };
                    };
                }
            }
        }
    }

    fn remove_at(&mut self, node: *mut Node) {
        unsafe {
            (*(*node).prev).next = (*node).next;
            (*(*node).next).prev = (*node).prev;
        }
    }

    fn push_mru(&mut self, node: *mut Node) {
        unsafe {
            (*node).next = self.mru;
            (*node).prev = (*self.mru).prev;
            (*(*self.mru).prev).next = node;
            (*self.mru).prev = node;
        }
    }

    fn pop_lru(&mut self) -> Option<*mut Node> {
        if self.len == 0 {
            return None;
        }

        unsafe {
            let node = (*self.lru).next;
            (*(*node).prev).next = (*node).next;
            (*(*node).next).prev = (*node).prev;
            Some(node)
        }
    }

    pub fn iter(&self) -> IterLRU {
        IterLRU {
            ptr: unsafe { &*self.mru }.prev,
            len: self.len,
        }
    }
}

impl Drop for LRUCache {
    fn drop(&mut self) {
        while let Some(node) = self.pop_lru() {
            let _ = unsafe { Box::from_raw(node) };
            self.len -= 1;
        }
        let _ = unsafe { Box::from_raw(self.mru) };
        let _ = unsafe { Box::from_raw(self.lru) };
    }
}

pub struct IterLRU {
    ptr: *mut Node,
    len: i32,
}

impl Iterator for IterLRU {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let Node {
            key,
            value,
            next: _,
            prev,
        } = *unsafe { &*self.ptr };
        self.ptr = prev;
        Some((key, value))
    }
}