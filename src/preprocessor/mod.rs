use std::collections::{BTreeMap, BTreeSet};
use std::iter::FromIterator;
use std::thread;

#[derive(Clone)]
pub struct Preprocessor {
    pub pattern_map: BTreeMap<Vec<u8>, u16>,
    pub reverse_pattern_map: BTreeMap<u16, Vec<u8>>,
    pub next_code: u16,
    max_pattern_length: usize,
    code_frequency: BTreeMap<u16, u32>,
    prediction_model: BTreeMap<Vec<u8>, u8>,
}

impl Preprocessor {
    pub fn new() -> Self {
        Preprocessor {
            pattern_map: BTreeMap::new(),
            reverse_pattern_map: BTreeMap::new(),
            next_code: 1,
            max_pattern_length: 4,
            code_frequency: BTreeMap::new(),
            prediction_model: BTreeMap::new(),
        }
    }

    pub fn serialize_dictionary(&self) -> Vec<u8> {
        let mut serialized = Vec::new();
        for (&code, pattern) in &self.reverse_pattern_map {
            serialized.extend(&code.to_be_bytes()); // Code to bytes
            serialized.push(pattern.len() as u8); // Length of the pattern
            serialized.extend(pattern); // The pattern itself
        }
        serialized
    }
    
    
    pub fn deserialize_dictionary(&mut self, serialized: &[u8]) {
        let mut i = 0;
        while i < serialized.len() {
            let code = u16::from_be_bytes([serialized[i], serialized[i+1]]);
            i += 2;
            let pattern_len = serialized[i] as usize;
            i += 1;
            let pattern = serialized[i..i + pattern_len].to_vec();
            i += pattern_len;
    
            self.pattern_map.insert(pattern.clone(), code);
            self.reverse_pattern_map.insert(code, pattern);
        }
    }
    
    pub fn preprocess(&mut self, data: &[u8]) -> Vec<u8> {
        self.max_pattern_length = self.determine_max_pattern_length(data);
        self.analyze_data(data);
        self.identify_patterns(data);
        self.build_prediction_model(data);
        self.parallel_transform_data(data)
    }

    pub fn determine_max_pattern_length(&self, data: &[u8]) -> usize {
        let unique_bytes = data.iter().collect::<BTreeSet<&u8>>().len();
        match unique_bytes {
            0..=16 => 2,  // Few unique bytes, shorter patterns might be better
            17..=32 => 3, // Moderate variety in bytes
            _ => 4        // High variety, longer patterns might be better
        }
    }
    
    pub fn analyze_data(&self, data: &[u8]) {
        let mut byte_frequency: BTreeMap<u8, usize> = BTreeMap::new();
    
        for &byte in data {
            *byte_frequency.entry(byte).or_insert(0) += 1;
        }
    
        // Print each byte's frequency
        for (byte, freq) in &byte_frequency {
            println!("Byte: {:?} ({}), Frequency: {}", *byte as char, byte, freq);
        }
    
        let entropy = self.calculate_entropy(&byte_frequency, data.len());
        println!("Data Entropy: {}", entropy);
    }
    

    fn calculate_entropy(&self, frequency: &BTreeMap<u8, usize>, total: usize) -> f64 {
        frequency.values().fold(0.0, |acc, &freq| {
            let probability = freq as f64 / total as f64;
            acc - probability * probability.log2()
        })
    }

    fn identify_patterns(&mut self, data: &[u8]) {
        let mut frequency_map: BTreeMap<Vec<u8>, u32> = BTreeMap::new();
    
        // Include single characters as well in the pattern identification
        for &byte in data {
            *frequency_map.entry(vec![byte]).or_insert(0) += 1;
        }
    
        for window_size in 2..=self.max_pattern_length {
            for window in data.windows(window_size) {
                *frequency_map.entry(window.to_vec()).or_insert(0) += 1;
            }
        }
    
        frequency_map.retain(|_, &mut freq| freq > 1);
    
        // Sort patterns
        let mut patterns: Vec<_> = frequency_map.into_iter().collect();
        patterns.sort_unstable_by(|(a_pattern, a_freq), (b_pattern, b_freq)| {
            b_freq.cmp(a_freq).then_with(|| a_pattern.cmp(b_pattern))
        });
    
        for (pattern, freq) in patterns.iter().take(254) {
            let code = self.next_code;
            self.next_code += 1;
            self.pattern_map.insert(pattern.clone(), code);
            self.reverse_pattern_map.insert(code, pattern.clone());
            self.code_frequency.insert(code, *freq);
            println!("Identified Pattern: {:?}, Code: {}, Frequency: {}", pattern, code, freq);
        }
    }
    
    fn build_prediction_model(&mut self, data: &[u8]) {
        let mut frequency_map: BTreeMap<Vec<u8>, u32> = BTreeMap::new();
        for window in data.windows(3) {
            *frequency_map.entry(window.to_vec()).or_insert(0) += 1;
        }

        self.prediction_model = BTreeMap::from_iter(
            frequency_map.into_iter()
                .filter(|&(_, freq)| freq > 1)
                .map(|(pattern, _)| (pattern[..2].to_vec(), pattern[2]))
        );
    }

    pub fn parallel_transform_data(&self, data: &[u8]) -> Vec<u8> {
        // self.transform_data(data)
        let num_threads = std::thread::available_parallelism().unwrap_or_else(|_| std::num::NonZeroUsize::new(1).unwrap()).get();
        let chunk_size = std::cmp::max(data.len() / num_threads, self.max_pattern_length);
        let mut threads = Vec::new();
    
        // Process each chunk in parallel
        for (index, chunk) in data.chunks(chunk_size).enumerate() {
            let chunk = chunk.to_vec();
            let preprocessor = self.clone();
    
            threads.push((index, thread::spawn(move || {
                preprocessor.transform_data(&chunk)
            })));
        }
    
        // Collect results, maintaining the order
        threads.sort_by_key(|&(index, _)| index);
        let mut transformed_data = Vec::new();
        for (_, thread) in threads {
            transformed_data.extend(thread.join().unwrap());
        }
    
        // No additional post-processing step is needed if the state is not altered during processing
        transformed_data
    }
    
    pub fn transform_data(&self, data: &[u8]) -> Vec<u8> {
        println!("--- Transforming data ---");
        let mut transformed_data = Vec::new();
        let mut i = 0;
    
        while i < data.len() {
            let mut found_match = false;
            for size in (2..=self.max_pattern_length.min(data.len() - i)).rev() {
                let pattern = &data[i..i + size];
                if let Some(&code) = self.pattern_map.get(pattern) {
                    println!("Pattern found: {:?}, Replacing with code: {}", pattern, code);
                    transformed_data.push(code as u8);
                    i += size;
                    found_match = true;
                    break;
                }
            }
            if !found_match {
                println!("No pattern found for byte: {}, Adding as is", data[i]);
                transformed_data.push(data[i]);
                i += 1;
            }
        }
        println!("Transformed data: {:?}", transformed_data);
        self.variable_length_encode(&transformed_data)
    }
    
    fn variable_length_encode(&self, data: &[u8]) -> Vec<u8> {
        let mut encoded_data = Vec::new();
        for &byte in data {
            if let Some(&code) = self.pattern_map.get(&vec![byte]) {
                let frequency = self.code_frequency.get(&code).unwrap_or(&1);
                let encoded_code = self.encode_code(code, *frequency);
                println!("Encoding byte: {}, Code: {}, Frequency: {}", byte, code, frequency);
                encoded_data.extend_from_slice(&encoded_code);
            } else {
                encoded_data.push(byte);
            }
        }
        // println!("Encoded data: {:?}", encoded_data);
        encoded_data
    }

    pub fn encode_code(&self, code: u16, frequency: u32) -> Vec<u8> {
        // Simplified variable-length encoding based on frequency
        if frequency > 100 {
            vec![code as u8] // More frequent patterns get shorter codes
        } else {
            vec![0xFF, code as u8] // Less frequent patterns get longer codes
        }
    }
    // ... additional methods as needed ...
    pub fn reverse_transform_data(&self, data: &[u8]) -> Vec<u8> {
        println!("--- Reverse transforming data ---");
        let mut decoded_data = Vec::new();
        let mut i = 0;
    
        while i < data.len() {
            if data[i] == 255 {
                println!("Prefix 255 found at index: {}", i);
                i += 1; // Skip the prefix
                let code = data[i] as u16;
                if let Some(pattern) = self.reverse_pattern_map.get(&code) {
                    println!("Index: {}, Decoding code: {} to pattern: {:?}", i, code, pattern);
                    decoded_data.extend_from_slice(pattern);
                }
            } else {
                let code = data[i] as u16;
                if let Some(pattern) = self.reverse_pattern_map.get(&code) {
                    println!("Index: {}, Decoding code: {} to pattern: {:?}", i, code, pattern);
                    decoded_data.extend_from_slice(pattern);
                } else {
                    println!("Index: {}, No pattern found for code: {}, treating as original byte", i, code);
                    decoded_data.push(data[i]);
                }
            }
            i += 1;
        }
        println!("Decoded data: {:?}", decoded_data);
        decoded_data
    } 
}
