use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn sum_integers_from_file(file_path: &str) -> Result<i32, io::Error> {
    // TODO: Implement this function
    // Hint: Use `File::open`, `BufReader::new`, and `.lines()` to process the file.
    // Use `?` to propagate errors and `io::Error::new` for custom errors.
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut sum = 0;
    for num_str in reader.lines() {
        let num = num_str
            .unwrap().parse::<i32>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid number"))?;
        sum += num;
    }
    Ok(sum)
}

// Example usage
pub fn main() {
    let file_path = "numbers.txt";

    match sum_integers_from_file(file_path) {
        Ok(sum) => println!("The sum is: {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}
