// CONTROL FLOW PART ONE

// 1- Conditions [if...else]

// Rust uses if...else conditions to make decisions based on the value of a boolean expression.

// Syntax:
// if condition {
//     // code to execute if the condition is true
// } else {
//     // code to execute if the condition is false
// }

// The else block is optional. If the condition is true, the code inside the if block will be executed. If the condition is false, the code inside the else block will be executed

// 2 - Repeating Actions [Loops]

// Rust provides three types of loops to repeat actions:
// 1- loop
// 2- while

#![allow(warnings)]
fn main() {
    // if true {
    //     println!("The condition is true");
    // } else {
    //     println!("The condition is false");
    // }

    // let age: u16 = 18;
    // if age >= 18 {
    //     println!("You can drive a car.");
    // } else {
    //     println!("You can't drive a car.");
    // }

    // Multiple conditions with else if
    // let number = 6;
    // if number % 4 == 0 {
    //     println!("The number is divisible by 4.");
    // } else if number % 3 == 0 {
    //     println!("The number is divisible by 3.");
    // } else if number % 2 == 0 {
    //     println!("The number is divisible by 2.");
    // } else {
    //     println!("The number is not divisible by 4, 3, or 2.");
    // }

    // Using if in a let statement
    // let condition = true;
    let condition = false;
    let number = if condition {
        5
     } else { 
        6
        // Below will give error because the type of the number is i32 and the type of the string is &str.
        // "six"
    };
    println!("Number: {number}");
}
