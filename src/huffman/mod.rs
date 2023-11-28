use std::collections::{BinaryHeap, HashMap, BTreeMap};
use std::cmp::Ordering;

use crate::adaptive_dictionary::AdaptiveDictionary;

#[derive(Debug)]
pub struct HuffmanNode {
    pub frequency: u32,
    pub value: u8,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn new(frequency: u32, value: u8, left: Option<Box<HuffmanNode>>, right: Option<Box<HuffmanNode>>) -> Self {
        HuffmanNode { frequency, value, left, right }
    }
}

#[derive(Debug)]
pub struct HuffmanTuple {
    frequency: u32,
    value: u8,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanTuple {
    fn new(frequency: u32, value: u8, left: Option<Box<HuffmanNode>>, right: Option<Box<HuffmanNode>>) -> Self {
        HuffmanTuple { frequency, value, left, right }
    }
}

impl Ord for HuffmanTuple {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for HuffmanTuple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for HuffmanTuple {}

impl PartialEq for HuffmanTuple {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

pub fn build_huffman_tree(data: &[u8]) -> Option<Box<HuffmanNode>> {
    let mut frequencies = HashMap::new();
    for &byte in data {
        *frequencies.entry(byte).or_insert(0) += 1;
    }

    // Print the frequencies for debugging
    // println!("Frequencies: {:?}", frequencies);

    let mut heap: BinaryHeap<HuffmanTuple> = frequencies.into_iter()
        .map(|(value, frequency)| {
            // println!("Inserting into heap: value={}, frequency={}", value, frequency);
            HuffmanTuple::new(frequency, value, None, None)
        })
        .collect();

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        // println!("Combining nodes: left=(value={}, freq={}), right=(value={}, freq={})", left.value, left.frequency, right.value, right.frequency);

        let merged_freq = left.frequency + right.frequency;
        heap.push(HuffmanTuple::new(merged_freq, std::cmp::min(left.value, right.value), Some(Box::new(HuffmanNode::new(left.frequency, left.value, left.left, left.right))), Some(Box::new(HuffmanNode::new(right.frequency, right.value, right.left, right.right)))));

        // Print the state of the heap after each merge
        println!("Heap after merge: {:?}", heap);
    }

    let root = heap.pop();
    println!("Final Huffman tree root: {:?}", &root.as_ref());

    root.map(|tuple| Box::new(HuffmanNode::new(tuple.frequency, tuple.value, tuple.left, tuple.right)))
}

pub fn generate_huffman_codes(node: &HuffmanNode, prefix: &mut Vec<u8>, codes: &mut BTreeMap<u8, Vec<u8>>) {
    if node.left.is_none() && node.right.is_none() {
        codes.insert(node.value, prefix.clone());
        return;
    }
    
    if let Some(ref left_node) = node.left {
        prefix.push(0);
        generate_huffman_codes(left_node, prefix, codes);
        prefix.pop();
    }

    if let Some(ref right_node) = node.right {
        prefix.push(1);
        generate_huffman_codes(right_node, prefix, codes);
        prefix.pop();
    }
}

pub fn build_huffman_tree_with_dictionary(dictionary: &AdaptiveDictionary) -> Option<Box<HuffmanNode>> {
    let mut heap = BinaryHeap::new();

    // Insert all characters and their frequencies into the heap
    for (&value, &frequency) in dictionary.get_frequencies() {
        heap.push(HuffmanTuple::new(frequency, value, None, None));
    }

    // Special case for when there is only one unique character
    if heap.len() == 1 {
        let single_node = heap.pop().unwrap();
        return Some(Box::new(HuffmanNode::new(single_node.frequency, single_node.value, None, None)));
    }

    // Combine nodes until there's only one node left (the root of the tree)
    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let merged_freq = left.frequency + right.frequency;
        heap.push(HuffmanTuple::new(merged_freq, std::cmp::min(left.value, right.value), Some(Box::new(HuffmanNode::new(left.frequency, left.value, left.left, left.right))), Some(Box::new(HuffmanNode::new(right.frequency, right.value, right.left, right.right)))));
    }

    // The remaining node in the heap is the root of the Huffman tree
    heap.pop().map(|tuple| Box::new(HuffmanNode::new(tuple.frequency, tuple.value, tuple.left, tuple.right)))
}

pub fn huffman_decode(encoded_data: &[u8], huffman_tree: &HuffmanNode) -> Vec<u8> {
    if encoded_data.is_empty() {
        return Vec::new();
    }

    let mut decoded_data = Vec::new();
    let mut current_node = huffman_tree;
    let bits_in_last_byte = encoded_data[encoded_data.len() - 1] as usize;

    for (index, &byte) in encoded_data.iter().enumerate().take(encoded_data.len() - 1) {
        let bits_to_process = if index == encoded_data.len() - 2 { bits_in_last_byte } else { 8 };

        for i in 0..bits_to_process {
            let bit = (byte >> (7 - i)) & 1;
            // println!("Decoding byte {}, bit {}: {}", index, i, bit);

            current_node = if bit == 0 {
                current_node.left.as_ref().unwrap()
            } else {
                current_node.right.as_ref().unwrap()
            };

            if current_node.left.is_none() && current_node.right.is_none() {
                decoded_data.push(current_node.value);
                current_node = huffman_tree;
            }
        }
    }

    decoded_data
}

pub fn huffman_encode(data: &[u8], codes: &BTreeMap<u8, Vec<u8>>) -> Vec<u8> {
    let mut encoded_data = Vec::new();
    let mut current_bitstring: Vec<u8> = Vec::new();

    // Encode the data into a bitstring
    for &byte in data {
        if let Some(code) = codes.get(&byte) {
            // println!("Encoding byte: {}, Code: {:?}", byte, code);
            current_bitstring.extend(code);
        }
    }

    // Process the bitstring into bytes
    let mut i = 0;
    while i + 8 <= current_bitstring.len() {
        let byte = current_bitstring[i..i + 8].iter().fold(0, |acc, &bit| (acc << 1) | bit);
        // println!("Encoded byte: {}", byte);
        encoded_data.push(byte);
        i += 8;
    }

    // Handle the last byte
    if i < current_bitstring.len() {
        let remaining_bits = &current_bitstring[i..];
        let mut last_byte = 0;
        for &bit in remaining_bits {
            last_byte = (last_byte << 1) | bit;
        }
        last_byte <<= 8 - remaining_bits.len(); // Pad the remaining bits
        // println!("Last encoded byte: {}", last_byte);
        encoded_data.push(last_byte);
    }

    // Append the number of bits in the last byte (including padding)
    let bits_in_last_byte = (current_bitstring.len() % 8) as u8;
    if bits_in_last_byte == 0 && !current_bitstring.is_empty() {
        // If the bitstring divides evenly by 8, the last byte is fully used
        encoded_data.push(8);
    } else {
        // Otherwise, record the actual number of bits used in the last byte
        encoded_data.push(bits_in_last_byte);
    }

    encoded_data
}
