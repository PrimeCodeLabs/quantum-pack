
#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use quantum_pack::{huffman::{build_huffman_tree, generate_huffman_codes, HuffmanNode, build_huffman_tree_with_dictionary, huffman_encode, huffman_decode}, adaptive_dictionary::AdaptiveDictionary};
    
    fn create_test_tree() -> Option<Box<HuffmanNode>> {
        let data = b"example data for adaptive dictionary";
        build_huffman_tree(data)
    }

    #[test]
    fn test_build_huffman_tree_non_empty_data() {
        let tree = create_test_tree();
        assert!(tree.is_some());
    }

    #[test]
    fn test_build_huffman_tree_empty_data() {
        let tree = build_huffman_tree(b"");
        assert!(tree.is_none());
    }

    #[test]
    fn test_build_huffman_tree_single_character() {
        let tree = build_huffman_tree(b"aaaaaa");
        assert!(tree.is_some());
        assert_eq!(tree.unwrap().frequency, 6);
    }

    #[test]
    fn test_generate_huffman_codes() {
        let tree = create_test_tree().unwrap();
        let mut codes = BTreeMap::new();
        generate_huffman_codes(&tree, &mut vec![], &mut codes);

        assert!(!codes.is_empty());
        assert!(codes.get(&b'e').is_some());
    }

    #[test]
    fn test_build_huffman_tree_with_dictionary() {
        let mut dictionary = AdaptiveDictionary::new();
        dictionary.update(b"example data for adaptive dictionary");
        let tree = build_huffman_tree_with_dictionary(&dictionary);

        assert!(tree.is_some());
    }

    #[test]
    fn test_huffman_decode_basic() {
        let tree = create_test_tree().unwrap();
        print!("{:?}", tree);
        let data = b"example";
        let mut codes = BTreeMap::new();
        generate_huffman_codes(&tree, &mut vec![], &mut codes);
        let encoded_data = huffman_encode(data, &codes);
        print!("{:?}", encoded_data);
        let decoded_data = huffman_decode(&encoded_data, &tree);

        assert_eq!(decoded_data, data);
    }

    #[test]
    fn test_huffman_encode_basic() {
        let tree = create_test_tree().unwrap();
        let data = b"example";
        let mut codes = BTreeMap::new();
        generate_huffman_codes(&tree, &mut vec![], &mut codes);

        let encoded_data = huffman_encode(data, &codes);
        assert!(!encoded_data.is_empty());
    }
}
