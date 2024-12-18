// Shadowing in Rust
// Shadowing is when you declare a variable with the same name as a previous variable. This is useful when you want to change the type of the value but keep the same name.

// Shadowing is different from using the mut keyword because we’ll get a compile-time error if we forget to reassign the variable. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

// The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.

// The variable is shadowed in the inner scope and the outer scope variable is not accessible in the inner scope.

fn main() {
    let x = 5;

    // x = 10; // This will give error because x is immutable by default.

    // You have to use the let keyword to shadow the variable.
    // The first x is shadowed by the second x.
    let x = x + 1;

    // When you have shadowed a variable, the outer variable is not accessible in the inner scope.
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x in the outer scope is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    // The below code will give error because the variable spaces is immutable after shadowing.
    // let mut = "   ";
    // let mut spaces = spaces.len();
}
