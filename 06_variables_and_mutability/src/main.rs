// Variables and Mutability
// Once a variable is declared as mutable, it can be changed.

fn main() {
    println!("Variables and Mutability");
    let a = 5;
    println!("The value of a is: {}", a);
    // Cannot assign twice to immutable variable
    // a = 6; // This will throw an error as a is immutable

    // you will have to declare a as mutable
    // let mut a = 5;

}