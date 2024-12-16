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
    // A tuple is a collection of values of different types.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let person: (&str, &str, i8) = ("John", "Doe", 30);
    println!("Name: {} {}", person.0, person.1);

    let my_mix_tuple = (1, 2.5, "Hello", true);
    println!("My mix tuple: {:?}", my_mix_tuple);

    // Slices: A slice is a reference to a contiguous sequence of elements in a collection.
    // Slices are used to give a section of an array or a whole array.
    // Slices do not have a fixed size.
    // Slices are used to borrow a section of an array.
    let number_slices: &[i32] = &[1,2,3,4,5];
    println!("Slice: {:?}", number_slices);

    let animal_slices: &[&str] = &["Dog", "Cat", "Cow", "Elephant"];
    println!("Animal slice: {:?}", animal_slices);

    let book_slices: &[&str] = &[&"IT".to_string(), &"Science".to_string(), &"Math".to_string(), &"History".to_string()];
    println!("Book slice: {:?}", book_slices);

    // Strings Vs String slices (&str)
    // Strings are stored in the heap.
    // Strings are mutable.
    // Strings are growable.
    // String slices are stored in the stack.
    // String slices are immutable.
    // String slices are fixed size.
    let mut stong_cold: String = String::from("Hell, ");
    println!("Stone Cold Says: {}", stong_cold);
    // You need to declare mut to make the string mutable!
    stong_cold.push_str("Yeah!");

    // &str is a string slice.
    // &str is a reference to a string.
    // &str is immutable.
    // when you want to work with string data that you don't need to modify, you can use string slices.
    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("String slice: {}", slice);

    // the stack is faster than the heap.
    
}

// The below function will not work because the slice is not defined in the function.
// fn print(){
//     println!("SLICE {}", slice);
// }