use std::fmt::Display;

// TODO: Define the generic function `compare_and_display` with appropriate trait bounds.
pub fn compare_and_display<T: Display + PartialOrd> (input1: T, input2: T) -> T {
     if PartialOrd::gt(&input1, &input2) {
          input1
     }else {
          input2
     }

}

// Example usage
pub fn main() {
     let greater = compare_and_display(10, 20);
     println!("Greater value: {}", greater);

     let greater = compare_and_display("Apple", "Orange");
     println!("Greater value: {}", greater);
}
