use std::{collections::BTreeMap, fs::File, io::{self, Read, Write}};
use crate::huffman::{HuffmanNode, build_huffman_tree_with_dictionary, generate_huffman_codes, huffman_decode, huffman_encode};
use crate::preprocessor::Preprocessor;
use crate::adaptive_dictionary::AdaptiveDictionary;
use std::convert::TryInto;
use std::str;

// This module handles the compression and decompression of data using Huffman coding
// and an adaptive dictionary-based preprocessor. The key aspects that need to be consistent
// across both compression and decompression processes are:
// 1. Frequency Table and Huffman Tree: For consistent encoding/decoding rules.
// 2. Processed Data: Ensuring data integrity post preprocessing.
// 3. Huffman Codes: Generated from the Huffman tree, crucial for encoding and decoding.
// 4. Serialized Frequency Table: Format and content should match in both compression and decompression.
// 5. Compressed Data: Output of compression and input for decompression.
// 6. Decompressed Data: Should match the original input data for lossless handling.

// Serialize the frequency table
pub fn serialize_frequency_table(dictionary: &AdaptiveDictionary) -> Vec<u8> {
    let mut serialized = Vec::new();
    for (&byte, &frequency) in dictionary.get_frequencies() {
        if frequency > 0 {
            serialized.push(byte); // Character byte
            serialized.extend_from_slice(&frequency.to_be_bytes()); // Frequency bytes
        }
    }
    serialized
}

// Deserialize the frequency table
pub fn deserialize_frequency_table(serialized: &[u8]) -> AdaptiveDictionary {
    let mut dictionary = AdaptiveDictionary::new();
    for chunk in serialized.chunks_exact(5) {
        let byte = chunk[0];
        let frequency = u32::from_be_bytes([chunk[1], chunk[2], chunk[3], chunk[4]]);
        dictionary.frequencies.insert(byte, frequency);
    }
    dictionary
}

// Compress data
pub fn compress(data: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut preprocessor = Preprocessor::new();
    let processed_data = preprocessor.preprocess(data);

    let mut dictionary = AdaptiveDictionary::new();
    dictionary.update(&processed_data);

    let huffman_tree = build_huffman_tree_with_dictionary(&dictionary).unwrap();

    let mut codes = BTreeMap::new();
    generate_huffman_codes(huffman_tree.as_ref(), &mut vec![], &mut codes);

    let huffman_encoded_data = huffman_encode(&processed_data, &codes);

    let frequency_table = serialize_frequency_table(&dictionary);

    let serialized_dictionary = preprocessor.serialize_dictionary();

    (huffman_encoded_data, frequency_table, serialized_dictionary)
}

// Decompress data
pub fn decompress(encoded_data: &[u8], frequency_table: &[u8], serialized_dictionary: &[u8], huffman_tree: &HuffmanNode) -> Vec<u8> {
    let huffman_decoded_data = huffman_decode(encoded_data, huffman_tree);

    let mut preprocessor = Preprocessor::new();
    preprocessor.deserialize_dictionary(serialized_dictionary);

    preprocessor.reverse_transform_data(&huffman_decoded_data)
}

// Compress a file
pub fn compress_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let mut file = File::open(input_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let (compressed, frequency_table, serialized_dictionary) = compress(&contents);

    let mut output_file = File::create(output_path)?;

    output_file.write_all(&(frequency_table.len() as u32).to_be_bytes())?;
    output_file.write_all(&frequency_table)?;
    output_file.write_all(&(serialized_dictionary.len() as u32).to_be_bytes())?;
    output_file.write_all(&serialized_dictionary)?;
    output_file.write_all(&compressed)?;

    Ok(())
}
// Decompress a file
pub fn decompress_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let mut file = File::open(input_path)?;
    let mut combined_contents = Vec::new();
    file.read_to_end(&mut combined_contents)?;

    // Read frequency table size and content
    let (size_bytes, rest) = combined_contents.split_at(4);
    let frequency_table_size = u32::from_be_bytes(size_bytes.try_into().unwrap()) as usize;
    let (frequency_table, rest) = rest.split_at(frequency_table_size);

    // Read serialized dictionary size and content
    let (size_bytes, rest) = rest.split_at(4);
    let dictionary_size = u32::from_be_bytes(size_bytes.try_into().unwrap()) as usize;
    let (serialized_dictionary, compressed_data) = rest.split_at(dictionary_size);

    let dictionary = deserialize_frequency_table(frequency_table);
    let huffman_tree = build_huffman_tree_with_dictionary(&dictionary).unwrap();

    let decompressed = decompress(compressed_data, frequency_table, serialized_dictionary, &huffman_tree);


    // Convert decompressed data to a string
    let decompressed_str = match str::from_utf8(&decompressed) {
        Ok(s) => s,
        Err(e) => {
            //println!("UTF-8 error at byte index: {}", e.valid_up_to());
            return Err(io::Error::new(io::ErrorKind::InvalidData, e));
        }
    };
    
    //println!("Final decompressed string: {:?}", decompressed_str);

    // Write the string to the output file
    let mut output_file = File::create(output_path)?;
    output_file.write_all(decompressed_str.as_bytes())?;

    Ok(())
}