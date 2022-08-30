use std::collections::HashMap;

#[derive(Default)]
pub struct Codec {
    m1: HashMap<String, u64>,
    m2: HashMap<u64, String>,
    count: u64,
}

/*
    535 - Encode and decode TinyURL
*/
impl Codec {
    pub fn new() -> Self {
        Self::default()
    }

    // O(1)
    pub fn encode(&mut self, longURL: String) -> String {
        match self.m1.get(&longURL) {
            Some(index) => index.to_string(),
            None => {
                self.m1.insert(longURL.clone(), self.count);
                self.m2.insert(self.count, longURL);
                self.count += 1;
                (self.count - 1).to_string()
            }
        }
    }

    // O(1)
    pub fn decode(&self, shortURL: String) -> String {
        let index = str::parse(&shortURL).ok().unwrap();
        self.m2.get(&index).cloned().unwrap()
    }
}
