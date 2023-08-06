// Scans source file and returns a buffer of characters as a vector
use std::fs;
use std::io::Error;

// Helper function for reading file -> string
// Returns an error type for result checking
fn read(file_path: &str) -> Result<String, Error> {
    return fs::read_to_string(file_path);
}

// Make sure no errors happened
pub fn scan(file_path: &str) -> Vec<char> {
    match read(file_path) {
        // If no errors occured, return the buffer with the characters (minus spaces)
        Ok(contents) => {
            let mut buffer: Vec<char> = contents.chars().collect();
            buffer.retain(|&c| c != ' ');

            println!("{:?}", buffer);
            return buffer;
        }
        // If errors occured, print out an error and return a blank vector
        Err(err) => {
            eprintln!("Error reading file: {}", err);

            return Vec::new();
        }
    }
}