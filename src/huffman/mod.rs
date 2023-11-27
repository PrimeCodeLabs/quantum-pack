use std::collections::{BinaryHeap, HashMap, BTreeMap};
use std::cmp::Ordering;

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

    let mut heap: BinaryHeap<HuffmanTuple> = frequencies.into_iter()
        .map(|(value, frequency)| HuffmanTuple::new(frequency, value, None, None))
        .collect();

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let merged_freq = left.frequency + right.frequency;
        heap.push(HuffmanTuple::new(merged_freq, std::cmp::min(left.value, right.value), Some(Box::new(HuffmanNode::new(left.frequency, left.value, left.left, left.right))), Some(Box::new(HuffmanNode::new(right.frequency, right.value, right.left, right.right)))));
    }

    heap.pop().map(|tuple| Box::new(HuffmanNode::new(tuple.frequency, tuple.value, tuple.left, tuple.right)))
}

pub fn generate_huffman_codes(node: &Option<Box<HuffmanNode>>, prefix: &mut Vec<u8>, codes: &mut BTreeMap<u8, Vec<u8>>) {
    if let Some(node) = node {
        if node.left.is_none() && node.right.is_none() {
            codes.insert(node.value, prefix.clone());
            return;
        }
        prefix.push(0);
        generate_huffman_codes(&node.left, prefix, codes);
        prefix.pop();
        prefix.push(1);
        generate_huffman_codes(&node.right, prefix, codes);
        prefix.pop();
    }
}
