pub mod huffman;
pub mod adaptive_dictionary;

pub mod preprocessor;
mod compression; // Import the new module
pub use compression::{compress, decompress, compress_file, decompress_file, deserialize_frequency_table, serialize_frequency_table};