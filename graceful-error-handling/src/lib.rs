pub fn parse_percentage(input: &str) -> Result<u8, String> {
    // TODO: Implement the function here
    let parse_result = input.parse::<u8>();
    if parse_result.is_ok() {
        let number = parse_result.unwrap();
        if number <= 100 {
            Ok(number)
        } else {
            Err(String::from("Percentage out of range"))
        }
    } else {
        Err(String::from("Invalid input"))
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    assert_eq!(result, Ok(50));

    let result = parse_percentage("101");
    assert_eq!(result, Err("Percentage out of range".to_string()));

    let result = parse_percentage("abc");
    assert_eq!(result, Err("Invalid input".to_string()));
}
