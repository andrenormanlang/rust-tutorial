// fn main() {
//     println!("Hello, 🦀 Rust from CARGO!");
// }

// Primitive data types in Rust
// int, float, bool, char

// Integer
// Rust has two types of integers: signed and unsigned.
// Signed integers can be positive, negative, or zero.
// Unsigned integers can only be positive or zero.
// Signed integers are represented using the i prefix, followed by the number of bits they use.
// Unsigned integers are represented using the u prefix, followed by the number of bits they use.
// The number of bits can be 8, 16, 32, 64, or 128.
// The default integer type is i32.
//  i8, i16, i32, i64, i128 : Signed integers
//  u8, u16, u32, u64, u128 : Unsigned integers

// fn main() {
//     let a: i8 = -10;
//     let b: i16 = 20;
//     let c: i32 = -30;
//     let d: i64 = 40;
//     let e: i128 = 50;

//     let f: u8 = 60;
//     let g: u16 = 70;
//     let h: u32 = 80;
//     let i: u64 = 90;
//     let j: u128 = 100;

//     println!("a: {}", a);
//     println!("b: {}", b);
//     println!("c: {}", c);
//     println!("d: {}", d);
//     println!("e: {}", e);
//     println!("f: {}", f);
//     println!("g: {}", g);
//     println!("h: {}", h);
//     println!("i: {}", i);
//     println!("j: {}", j);

//     println!("Maximum value of i64: {}", i);
// }

// Compound data types in Rust
// Tuple, Array, Slice and Strings(slice string)

// Arrays
fn main() {

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // {:#?} is used to print the array in a pretty format
    // {:?} is used to print the array in a single line
    println!("Numbers: {:?}", numbers);
    println!("First element: {}", numbers[0]);
    println!("Second element: {}", numbers[1]);

    // Tuples
    let person: (&str, &str, i8) = ("John", "Doe", 30);
    println!("Name: {} {}", person.0, person.1);

    let my_mix_tuple = (1, 2.5, "Hello", true);
    println!("My mix tuple: {:?}", my_mix_tuple);

    // Slices: A slice is a reference to a contiguous sequence of elements in a collection.
    let number_slices: &[i32] = &[1,2,3,4,5];
    println!("Slice: {:?}", number_slices);

    let animal_slices: &[&str] = &["Dog", "Cat", "Cow", "Elephant"];
    println!("Animal slice: {:?}", animal_slices);

    let book_slices: &[&str] = &["Rust", "Python", "Java", "C++"];
    println!("Book slice: {:?}", book_slices);
}
    