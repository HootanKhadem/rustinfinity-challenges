use std::fs::File;
use std::io::Read;

pub fn read_file(file_path: &str) -> Option<String> {
    // Hint: Use `File::open` and `.read_to_string()` with `?` to propagate errors.
    let mut file_contents = String::new();
    let mut result = File::open(file_path).ok()?;
    let _ = result.read_to_string(&mut file_contents);
    Some(file_contents)
}

// Example usage
pub fn main() {
    let file_path = "example.txt";

    match read_file(file_path) {
        Some(contents) => println!("File contents:\n{}", contents),
        None => println!("Failed to read the file."),
    }
}
