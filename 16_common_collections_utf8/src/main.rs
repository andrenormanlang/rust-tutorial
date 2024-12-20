

// UTF-8
// Rust strings are UTF-8 encoded. This means that Rust strings can contain any valid Unicode code point, including emojis and other non-ASCII characters. Rust strings are stored as a sequence of bytes, where each byte represents a single Unicode code point. This makes Rust strings very flexible and powerful, as they can represent text in any language or script.

// Rust provides several ways to work with strings, including the String type, the str type, and the char type. The String type is a growable, heap-allocated string that is owned by the program. The str type is a fixed-size, stack-allocated string slice that is borrowed from another string. The char type represents a single Unicode code point.

fn main() {
    // -----------------------------------UTF8-----------------------------------
    // 1
    let _s = "whatever".to_string();
    // 2
    let _s = String::from("whatever");
    // Mutate the value [push to it]
    let mut _s = String::from("foo");
    _s.push_str("bar");
    _s.push('!');

    println!("The value of s is: {}", _s);

    
    // If you want to combine two strings, you can use the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("The value of s3 is: {}", s3);
    
    // Formatting Strings
    let salam =String::from("приветствие");
    let salut =String::from("Salut");
    let full_message = format!("{salam} {salut}");
    println!( "{full_message}");

}
