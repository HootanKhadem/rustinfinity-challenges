// 1. Define the struct
pub struct TextFinder<'a> {
    string_slice: &'a str,
}

// 2. Implement the struct and define the methods
impl<'a> TextFinder<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            string_slice: text,
        }
    }

    pub fn find_first(&self, keyword: &str) -> Option<&str> {
        let lines = self.string_slice.lines();
        for line in lines {
            if line.contains(keyword) {
                return Some(line);
            }
        }
        None
    }

    pub fn find_many(&self, keyword: &str) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::new();
        let lines = self.string_slice.lines();
        for line in lines {
            if line.contains(keyword) {
                result.push(line);
            }
        }
        result
    }
}
// Example usage
pub fn main() {
    let text = "Rust is fast and memory-efficient.\nOwnership is key to Rust's safety.\nRustaceans love the borrow checker.";
    let finder = TextFinder::new(text);

    let first = finder.find_first("Rust");
    println!("{:?}", first); // Should print: Some("Rust is fast and memory-efficient.")

    let matches = finder.find_many("Rust");
    println!("{:?}", matches); // Should print: ["Rust is fast and memory-efficient.", "Ownership is key to Rust's safety."]
}
