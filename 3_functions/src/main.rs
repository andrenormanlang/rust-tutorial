// Functions in Rust
// Functions are a set of statements that perform a specific task.
// Functions are defined using the fn keyword.
// The main function is the entry point of a Rust program.



fn main() {
    hello_world();
    tell_height(180);
    human_id("André", 47, 180.5); 

    // Expressions 
    let _x:i32  = {
        let price: i32 = 100;
        let quantity: i32 = 5;
        price * quantity
    };
    println!("The total price is {}", _x);

   let y = add(5, 10);
    println!("The value of y is: {}", y);


    // Statements
    // Calling the BMI function from below
    let weight = 94.0;
    let height = 1.78;
    let bmi = calculate_bmi( weight, height);
    println!("Your BMI is: {:.2}", bmi);
}

// Hoisting in Rust
    // Can call a function before it is defined and anywhere in your code.
fn hello_world() {
    println!("Hello, RUST 🦀!");
}

fn tell_height(height: i32) {
    println!("My height is {} cm", height);
}

// You can insert more than one value
fn human_id(name: &str, age: u32, height: f32) {
    println!("My name is {}, I am {} years old and my height is {} cm", name, age, height); 
}

// Expressions and Statements


// Expressions everything that returns a value.
// Statements are instructions that perform some action and do not return a value.

// Expression:
// 5
// true & false
//  add(5, 10)
// if condition {value1} else {value2}
// ({code})

// Every value declared out of the main function should be declared with the const keyword.
// let x ={
//     // code
// }
// 
// const _x ={
//     // code
// };
// CHECK MAIN FUNCTION ABOVE FOR EXAMPLES

fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Statements
// Almost all statements end with a semicolon ;
// Statements do not return a value.
// let y = let x = 5; // This will not work because let x = 5; is a statement and it does not return a value.
// 1 Variable declarations : let x = 5;
// 2 Function definitions: fn foo() {}
// 3 Control flow statements: if condition {/*code */} else {/*code */}, while condition {/*code */}, etc..

// BMI = height(kg) / height(m)²
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}