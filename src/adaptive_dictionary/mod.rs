use std::collections::HashMap;

pub struct AdaptiveDictionary {
    map: HashMap<Vec<u8>, u32>,
    threshold: u32,
}

impl AdaptiveDictionary {
    pub fn new(threshold: u32) -> Self {
        AdaptiveDictionary {
            map: HashMap::new(),
            threshold,
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        // Temporary map to count frequencies
        let mut temp_map = HashMap::new();

        // Update the frequency map based on the input data
        for window in data.windows(2) {
            *temp_map.entry(window.to_vec()).or_insert(0) += 1;
        }

        // Merge temp_map into self.map
        for (key, count) in temp_map {
            *self.map.entry(key).or_insert(0) += count;
        }

        // Collect keys that are below the threshold
        let keys_to_remove: Vec<_> = self.map.iter()
            .filter(|&(_, &count)| count < self.threshold)
            .map(|(key, _)| key.clone())
            .collect();

        // Remove the low-frequency keys
        for key in keys_to_remove {
            self.map.remove(&key);
        }
    }

    pub fn compress(&self, data: &[u8]) -> Vec<u8> {
        let mut compressed = Vec::new();
        let mut i = 0;
        while i < data.len() {
            let mut found = false;
            for size in (2..=data.len() - i).rev() {
                let window = &data[i..i+size];
                if self.map.contains_key(window) {
                    compressed.push(b'#'); // Using '#' as a placeholder for compressed sequence
                    i += size;
                    found = true;
                    break;
                }
            }
            if !found {
                compressed.push(data[i]);
                i += 1;
            }
        }
        compressed
    }

    // Additional utility methods can be added here
    pub fn decompress(&self, data: &[u8]) -> Vec<u8> {
        let mut decompressed = Vec::new();
        let mut i = 0;
        while i < data.len() {
            if data[i] == b'#' {
                let mut found = false;
                for size in (2..=data.len() - i).rev() {
                    let window = &data[i..i+size];
                    if self.map.contains_key(window) {
                        decompressed.extend_from_slice(window);
                        i += size;
                        found = true;
                        break;
                    }
                }
                if !found {
                    decompressed.push(data[i]);
                    i += 1;
                }
            } else {
                decompressed.push(data[i]);
                i += 1;
            }
        }
        decompressed
    }

    pub fn get_map(&self) -> &HashMap<Vec<u8>, u32> {
        &self.map
    }

    pub fn get_threshold(&self) -> u32 {
        self.threshold
    }

    pub fn set_threshold(&mut self, threshold: u32) {
        self.threshold = threshold;
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}
