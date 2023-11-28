use std::{env, process};

use quantum_pack::{compress_file, decompress_file};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} [compress|decompress] <input file> <output file>", args[0]);
        process::exit(1);
    }

    match args[1].as_str() {
        "compress" => {
            let input_path = &args[2];
            let output_path = &args[3];
            compress_file(input_path, output_path).expect("Error compressing file");
        }
        "decompress" => {
            let input_path = &args[2];
            let output_path = &args[3];
            println!("{:?}", input_path);
            decompress_file(input_path, output_path).expect("Error decompressing file");
        }
        _ => {
            eprintln!("Invalid command. Use 'compress' or 'decompress'.");
            process::exit(1);
        }
    }
}

