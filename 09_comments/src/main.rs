// Comments in Rust
// Comments in Rust are similar to comments in other programming languages. You can use comments to explain your code and make it more readable.

// Single line comments
// Single line comments in Rust start with two forward slashes (//). The compiler ignores everything from // to the end of the line.

// Multi-line comments
// Multi-line comments in Rust start with /* and end with */. The compiler ignores everything between /* and */.

// Documentation comments
// Documentation comments in Rust start with /// or //! and are used to generate documentation for your code. Documentation comments are used to generate documentation for your code using the rustdoc tool.

// The rustdoc tool is a built-in tool in Rust that generates documentation for your code in HTML format. You can use the rustdoc tool to generate documentation for your code by running the following command in your terminal:

// rustdoc src/main.rs

// The above command will generate documentation for your code in HTML format and save it in the target/doc directory.



fn main() {
    // This is a single line comment. 
    println!("Hello, world!");
    println!("I feel lucky.");

    /* This is a multi-line / block commenting comment.
    In Rust, the idiomatic way to write comments is to use single line comments for short comments and multi-line comments for longer comments.
    */



}
