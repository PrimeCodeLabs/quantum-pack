use std::collections::BTreeMap;


pub struct AdaptiveDictionary {
    pub frequencies: BTreeMap<u8, u32>,
}

impl AdaptiveDictionary {
    pub fn new() -> Self {
        AdaptiveDictionary {
            frequencies: BTreeMap::new(),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        for &byte in data {
            *self.frequencies.entry(byte).or_insert(0) += 1;
        }
    }

    pub fn get_frequencies(&self) -> &BTreeMap<u8, u32> {
        &self.frequencies
    }
}
