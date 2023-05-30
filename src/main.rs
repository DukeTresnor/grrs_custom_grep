fn main() {
    println!("In progress");

    // std::env::args()
    // nth()
    // except()
    let string_pattern = std::env::args().nth(1).expect("Please provide a desired string pattern");
    let file_path = std::env::args().nth(2).expect("Please provide a file path");
}
