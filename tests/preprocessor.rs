use quantum_pack::preprocessor::Preprocessor;
#[test]
fn test_basic_functionality() {
    let mut preprocessor = Preprocessor::new();
    let data = b"The quick brown fox jumps over the lazy dog";
    let processed = preprocessor.preprocess(data);
    assert!(!processed.is_empty());
}

#[test]
fn test_empty_input() {
    let mut preprocessor = Preprocessor::new();
    let data = b"";
    let processed = preprocessor.preprocess(data);
    assert!(processed.is_empty());
}

#[test]
fn test_pattern_recognition() {
    let mut preprocessor = Preprocessor::new();
    let data = b"aaabbbccc";  // Repeating patterns
    let processed = preprocessor.preprocess(data);
    assert!(!processed.is_empty());

    // Since we know that repeating patterns like 'aaa', 'bbb', 'ccc' should be compressed,
    // we can check if the length of the processed data is less than the original
    assert!(processed.len() < data.len());
}

#[test]
fn test_variance_based_pattern_length() {
    let preprocessor = Preprocessor::new();
    let low_variance_data = [1u8; 100];  // Low variance
    let high_variance_data = (0u8..100).collect::<Vec<u8>>();  // High variance

    assert_eq!(preprocessor.determine_max_pattern_length(&low_variance_data), 2);
    assert_eq!(preprocessor.determine_max_pattern_length(&high_variance_data), 4);
}

#[test]
fn test_entropy_analysis() {
    let preprocessor = Preprocessor::new();
    let data = b"some random data";
    preprocessor.analyze_data(data);

    // This test would primarily ensure that `analyze_data` runs without panicking.
    // Assertions would be limited as the function does not return a value but logs the entropy.
    assert!(true); // Placeholder assertion
}

#[test]
fn test_parallel_processing_consistency() {
    let preprocessor = Preprocessor::new();
    let data = b"Data that is long enough to be split across threads";
    let processed_parallel = preprocessor.parallel_transform_data(data);
    let processed_sequential = preprocessor.transform_data(data);

    assert_eq!(processed_parallel, processed_sequential);
}

#[test]
fn test_encode_code_high_frequency() {
    let preprocessor = Preprocessor::new();
    let encoded = preprocessor.encode_code(10, 101);
    assert_eq!(encoded, vec![10]);
}

#[test]
fn test_encode_code_low_frequency() {
    let preprocessor = Preprocessor::new();
    let encoded = preprocessor.encode_code(10, 50);
    assert_eq!(encoded, vec![0xFF, 10]);
}

#[test]
fn test_reverse_transform_data_basic() {
    let mut preprocessor = Preprocessor::new();
    preprocessor.reverse_pattern_map.insert(1, vec![97, 98]); // 'ab' pattern
    let data = vec![1];
    let decoded = preprocessor.reverse_transform_data(&data);
    assert_eq!(decoded, vec![97, 98]);
}

#[test]
fn test_reverse_transform_data_no_pattern() {
    let preprocessor = Preprocessor::new();
    let data = vec![1, 2, 3];
    let decoded = preprocessor.reverse_transform_data(&data);
    assert_eq!(decoded, data);
}

#[test]
fn test_reverse_transform_data_various_patterns() {
    let mut preprocessor = Preprocessor::new();
    preprocessor.reverse_pattern_map.insert(1, vec![97]); // 'a'
    preprocessor.reverse_pattern_map.insert(2, vec![98]); // 'b'
    let data = vec![1, 2, 1, 2];
    let decoded = preprocessor.reverse_transform_data(&data);
    assert_eq!(decoded, vec![97, 98, 97, 98]);
}

#[test]
fn test_preprocessor_simple_data() {
    let mut preprocessor = Preprocessor::new();
    let data = b"AAAABBBBCCCCAAAABBBB"; // Simple repeating pattern

    // Compress the data
    let compressed = preprocessor.preprocess(data);
    assert_ne!(compressed, data.to_vec());

    // Reverse map for decompression
    for (pattern, code) in &preprocessor.pattern_map {
        preprocessor.reverse_pattern_map.insert(*code, pattern.clone());
    }

    // Decompress the data
    let decompressed = preprocessor.reverse_transform_data(&compressed);
    assert_eq!(decompressed, data);
}

#[cfg(test)]
mod tests {
    use quantum_pack::preprocessor::{Preprocessor, self};


    #[test]
    fn test_preprocessing_determinism() {
        let input_data = b"Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.";
        let mut preprocessor = Preprocessor::new();
        let mut preprocessor2 = Preprocessor::new();

        let first_run = preprocessor.preprocess(input_data);
        let second_run = preprocessor2.preprocess(input_data);

        assert_eq!(first_run, second_run, "Preprocessing should be deterministic");
    }

    #[test]
    fn test_reverse_preprocessing_determinism() {
        let input_data = b"Example data for testing determinism in reverse preprocessing";
        let mut preprocessor = Preprocessor::new();
        let processed_data = preprocessor.preprocess(input_data);

        let first_run = preprocessor.reverse_transform_data(&processed_data);
        let second_run = preprocessor.reverse_transform_data(&processed_data);

        assert_eq!(first_run, second_run, "Reverse preprocessing should be deterministic");
    }

    #[test]
    fn test_full_cycle_determinism() {
        let input_data = b"Example data for testing determinism in full cycle";
        let mut preprocessor = Preprocessor::new();

        let processed_data = preprocessor.preprocess(input_data);
        let recovered_data = preprocessor.reverse_transform_data(&processed_data);

        assert_eq!(input_data.to_vec(), recovered_data, "Full cycle (preprocess and reverse) should be deterministic and lossless");
    }

    #[test]
    fn test_pattern_overlaps() {
        let mut preprocessor = Preprocessor::new();
        let data = b"ababcabcd"; // Overlapping patterns 'ab', 'abc', and 'abcd'
        let processed = preprocessor.preprocess(data);
    
        // Check if patterns are correctly recognized and compressed.
        // The exact assertion will depend on how your algorithm is designed to handle overlaps.
        // This is a placeholder for the type of assertion you might use.
        assert!(processed.len() < data.len(), "Data should be compressed with overlapping patterns recognized");
    }
    #[test]
    fn test_space_character_handling() {
        let mut preprocessor = Preprocessor::new();
        let data = b"Rescuers in India have freed 41 workers who had been trapped in a collapsed Himalayan tunnel for 17 days. Miners drilled the final section by hand to reach the workers in the";
        let processed = preprocessor.preprocess(data);
    
        // Check if the space character is properly compressed.
        // The exact assertion depends on your algorithm's behavior.
        assert!(!processed.contains(&b' '), "Space characters should be compressed");
    
        // Decompression test
        let decompressed = preprocessor.reverse_transform_data(&processed);
        assert_eq!(decompressed, data, "Decompressed data should match original, including spaces");
    }
    
    
}
