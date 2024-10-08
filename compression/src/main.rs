use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

const OUTPUT_PATH: &str="output.gz";

#[warn(dead_code)]
fn compress_file(input_path: &str, output_path: &str) -> io::Result<()> {
    // Open the input file for reading
    let mut input_file = File::open(input_path)?;
    // Open or create the output file for writing compressed data
    let output_file = File::create(output_path)?;

    // Create a GzEncoder for compression
    let mut encoder = GzEncoder::new(output_file, Compression::default());
    // Buffer to hold file data
    let mut buffer = Vec::new();
    // Read the input file content into the buffer
    input_file.read_to_end(&mut buffer)?;

    // Write compressed data to the encoder (output file)
    encoder.write_all(&buffer)?;

    // Finish the encoding process and close the encoder
    encoder.finish()?;

    Ok(())
}

fn compress_file_from_terminal() -> io::Result<()> {
    // Collect the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid number of arguments",
        ));
    }
    let input_path = &args[1]; // First argument: input file path

    // Compress the input file and write to the output path
    compress_file(input_path, OUTPUT_PATH)?;

    Ok(())
}
fn main() {
    // compress_file(
    //     "/Users/mahesh/Desktop/Projects/Rust/practice/compression/src/test.txt",
    //     "output.txt.gz",
    // )
    // .unwrap()

    let res = compress_file_from_terminal();
    println!("{:?}", res)
}
