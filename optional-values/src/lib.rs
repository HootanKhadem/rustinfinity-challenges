pub fn find_first_even(numbers: &[i32]) -> Option<i32> {
    // Your code here...
    let mut found_number : Option<i32> = None;
    for number in numbers.iter() {
        if number % 2 == 0 {
            found_number = Some(*number);
            break;
        }
    }
    found_number
}

// Example usage
pub fn main() {
    let nums1 = vec![1, 3, 5, 8];
    let nums2 = vec![1, 3, 5];

    println!("{:?}", find_first_even(&nums1)); // Output: Some(8)
    println!("{:?}", find_first_even(&nums2)); // Output: None
}
