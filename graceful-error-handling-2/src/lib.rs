use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

// 1. Finish the definition
#[derive(Debug, PartialEq)]
pub enum ParsePercentageError {
    InvalidInput,
    OutOfRange,
}

impl Display for ParsePercentageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsePercentageError::InvalidInput => write!(f, "Invalid input"),
            ParsePercentageError::OutOfRange => write!(f, "Percentage out of range"),
        }
    }
}

// 2. Implement the `Error` trait
impl Error for ParsePercentageError {}

pub fn parse_percentage(input: &str) -> Result<u8, ParsePercentageError> {
    // 3. Implement this function
    let parse_result = input.parse::<u8>();
    if parse_result.is_ok() {
        let value = parse_result.unwrap();
        if value <= 100 {
            Ok(value)
        } else {
            Err(ParsePercentageError::OutOfRange)
        }
    } else {
        Err(ParsePercentageError::InvalidInput)
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    println!("{:?}", result); // Should print: Ok(50)

    let result = parse_percentage("101");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::OutOfRange)

    let result = parse_percentage("abc");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::InvalidInput)
}
