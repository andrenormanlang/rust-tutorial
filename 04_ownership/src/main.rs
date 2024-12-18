// Ownership, Borrowing, and References

// Ownership
// Ownership is Rust's most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector. Therefore, it's important to understand how ownership works in Rust.


// C, C++ - Memory Management control Issue
// Garbage Collector solves the problem but it has its own issues which is slow down the performance of the application.
//  [Stopping, Marking, Sweeping, Compacting]

// Ownership fixes the problem of memory management in Rust.
// Ownership is a feature of Rust that allows Rust to manage memory efficiently without needing a garbage collector.
// Ownership is a set of rules that the compiler checks at compile time.
// Ownership rules:
// 1. Each value in Rust has a variable that's called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.
// Ownership is Rust's most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector. Therefore, it's important to understand how ownership works in Rust.

// Example 1: Each value in Rust has a variable that's its owner.

// fn main() {
//     let s1 = String::from("RUST");
//     // Passing a reference to the owener of the value.
//     let len = calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, len);   
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// Example 2: There can only be one owner at a time.
// fn main() {
//     let s1 = String::from("RUST");
//     let s2 = s1;
//     // Below line will give an error because s1 is no longer valid.
//     // println!("{}, world!", s1);
//     // So we can use s2 instead of s1.
//     println!("{}, world!", s2);
// }

// Example 3: When the owner goes out of scope, the value will be dropped.
fn main() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}// s1 goes out of scope here, and its value will be dropped.

// s1 is no longer valid here, because it's out of scope.
// fn printLost(s: &string) {
//     println!("{}", &s1);
// }

fn calculate_length(s: &String) -> usize {
    s.len()
}