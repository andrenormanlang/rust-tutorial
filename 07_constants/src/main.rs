// Constants
// Variables that are bound to a name and do not change their value.
// Constants are not allowed to use mut keyword.
// Constants are declared using the const keyword.

fn main() {
    println!("Hello, world!");
    let mut x = 5;
    println!("The value of x is: {}", x);
    // const mut y = 10;
    // the below line will give error because constants are not allowed to use mut keyword and it should be in capital letters and type annotated.
    // println!("The value of y is: {}", y);
    const Y: i32 = 10;
    println!("The value of Y is: {}", Y);

    // Constants can be declared in any scope, including the global scope, which makes it a global constant.
    println!("The value of PI is: {}", PI);

    println!("The value of 3 hours in seconds is: {}", THREE_HOURS_IN_SECONDS);
}

// You can always declare a constant with type annotation in any scope, including the global scope, which makes it a global constant.
const PI: f64 = 3.14159265359;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;