#![allow(unused)]

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    string_pattern: String,
    // The file path to read that contains the target file
    file_path: std::path::PathBuf,
}

fn main() {
    println!("In progress");

    let args = Cli::parse();

    // args is a Cli -- Cli have string_pattern and file_path
    println!("String pattern: {}", args.string_pattern);
    println!("File path: {}", args.file_path.display());


    // std::env::args()
    // nth()
    // except()
    //let string_pattern = std::env::args().nth(1).expect("Please provide a desired string pattern");
    //let file_path = std::env::args().nth(2).expect("Please provide a file path");
}
