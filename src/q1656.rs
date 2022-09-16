/*
    1656 - Design Ordered Stream
    Time: O(n)
*/
pub struct OrderedStream {
    buffer: Vec<Option<String>>,
    index: usize,
}

impl OrderedStream {
    pub fn new(n: i32) -> Self {
        let n = n as usize;
        Self {
            buffer: vec![None; n],
            index: 0,
        }
    }

    pub fn insert(
        &mut self,
        id_key: i32,
        value: String,
    ) -> Vec<String> {
        let id = id_key as usize - 1;
        self.buffer[id] = Some(value);
        let mut res = vec![];
        while let Some(entry) =
            self.buffer.get_mut(self.index)
        {
            if let Some(value) = entry.take() {
                res.push(value);
                self.index += 1;
            } else {
                break;
            }
        }
        res
    }
}
