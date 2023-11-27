

#[test]
fn test_huffman_tree_building() {
    let data = b"test data";
    let tree = quantum_pack::huffman::build_huffman_tree(data);

    assert!(tree.is_some(), "Tree should be created for non-empty data");

    if let Some(node) = tree {
        // Assuming ' ' (space) is the most frequent character in "test data"
        assert_eq!(node.frequency, data.len() as u32, "Root node frequency should be equal to the length of input data");
        assert_eq!(node.value, b' ', "The most frequent character should be at the root for this specific input");
    }
}

#[test]
fn test_huffman_code_generation() {
    let data = b"test data";
    let tree = quantum_pack::huffman::build_huffman_tree(data);

    let mut codes = std::collections::BTreeMap::new();
    quantum_pack::huffman::generate_huffman_codes(&tree, &mut vec![], &mut codes);

    assert!(!codes.is_empty(), "Codes should be generated for non-empty data");

    // Check for specific known characters (these checks depend on your specific Huffman tree structure for "test data")
    if let Some(code) = codes.get(&b'e') {
        // Check the length or pattern of the code for 'e', for example
        assert_eq!(code.len(), 3, "Code for 'e' should be 4 bits long");
    }

    // Check for uniqueness of the codes
    let unique_codes: std::collections::HashSet<_> = codes.values().collect();
    assert_eq!(codes.len(), unique_codes.len(), "All Huffman codes should be unique");
}
