use std::collections::BTreeMap;

use quantum_pack::{huffman::{ build_huffman_tree, generate_huffman_codes, huffman_encode, huffman_decode}, adaptive_dictionary::AdaptiveDictionary, preprocessor::Preprocessor, serialize_frequency_table, deserialize_frequency_table};

#[test]
fn test_huffman_with_preprocessor_integration() {
    let input_data = b"The quick brown fox jumps over the lazy dog";

    // Step 1: Preprocess the data
    let mut preprocessor = Preprocessor::new();
    let processed_data = preprocessor.preprocess(input_data);

    // Step 2: Build Huffman Tree and generate codes
    let mut dictionary = AdaptiveDictionary::new();
    dictionary.update(&processed_data);
    let huffman_tree = build_huffman_tree(&processed_data).unwrap();
    let mut codes = BTreeMap::new();
    generate_huffman_codes(&huffman_tree, &mut Vec::new(), &mut codes);

    // Step 3: Encode the data using Huffman codes
    let encoded_data = huffman_encode(&processed_data, &codes);
    
    // Step 4: Decode the data
    let decoded_data = huffman_decode(&encoded_data, &huffman_tree);

    // Step 5: Reverse preprocess the data
    let original_data = preprocessor.reverse_transform_data(&decoded_data);

    // Step 6: Compare the final output with the original input
    assert_eq!(original_data, input_data);
}

mod tests {
    use quantum_pack::{deserialize_frequency_table, serialize_frequency_table, adaptive_dictionary::AdaptiveDictionary, compress_file, decompress_file};
    use std::{fs::{self, File}, io::{self, Read}};

    #[test]
    fn test_serialize_frequency_table() {
        let mut dictionary = AdaptiveDictionary::new();
        dictionary.frequencies.insert(97, 3); // 'a' = 3
        dictionary.frequencies.insert(98, 2); // 'b' = 2
        dictionary.frequencies.insert(99, 1); // 'c' = 1

        let serialized = serialize_frequency_table(&dictionary);
        println!("{:?}", serialized);
        // Expected: [97, 3, 0, 0, 0, 98, 2, 0, 0, 0, 99, 1, 0, 0, 0]
        let expected = [
            97, 0, 0, 0, 3, 98, 0, 0, 0, 2, 99, 0, 0, 0, 1,
        ];

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_deserialize_frequency_table() {
        let data = [
            97, 0, 0, 0, 3, 98, 0, 0, 0, 2, 99, 0, 0, 0, 1,
        ];

        let dictionary = deserialize_frequency_table(&data);

        assert_eq!(*dictionary.frequencies.get(&97).unwrap(), 3); // 'a' = 3
        assert_eq!(*dictionary.frequencies.get(&98).unwrap(), 2); // 'b' = 2
        assert_eq!(*dictionary.frequencies.get(&99).unwrap(), 1); // 'c' = 1
    }

    #[test]
    fn test_compress_decompress_file() -> io::Result<()> {
        let input_path = "./test.txt"; // Path to the test file
        let compressed_path = "./compressedfile.zip"; // Path for the compressed output
        let decompressed_path = "./decompressedfile.txt"; // Path for the decompressed output

        // Compress the file
        compress_file(input_path, compressed_path)?;

        // Decompress the file
        decompress_file(compressed_path, decompressed_path)?;

        // Read the original and decompressed files and compare their contents
        let mut original_file = File::open(input_path)?;
        let mut original_contents = String::new();
        original_file.read_to_string(&mut original_contents)?;

        let mut decompressed_file = File::open(decompressed_path)?;
        let mut decompressed_contents = String::new();
        decompressed_file.read_to_string(&mut decompressed_contents)?;

        assert_eq!(original_contents, decompressed_contents, "The original and decompressed contents should be the same");

        // Clean up: Remove the compressed and decompressed files after the test
        fs::remove_file(compressed_path)?;
        fs::remove_file(decompressed_path)?;

        Ok(())
    }
}