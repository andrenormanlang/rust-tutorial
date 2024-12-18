// Repetition with Loops
// Doing things over and over

// Loop is the simplest way to repeat an action in Rust. It runs the code inside the loop block until you explicitly tell it to stop.
// 1- loop
// 2- while
// 3- for

// 1- loop
// The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.


fn main() {
    // Loop keyword
    // loop {
    //     println!("Hello, world!");
    // }

    // Loop with break
    // let mut counter = 0;
    
    // let result = loop {
    //     counter += 1;
    //     if counter == 20 {
    //         break counter - 100;
    //     }
    // };
    // println!("The result is: {result}");

    // Loop labels to disambiguate between multiple nested loops
    let mut count = 0;
    // Outer loop - counting_up
    // The 'counting_up label is used to break the outer loop. So the inner loop will not be executed.
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;
    //     // Inner loop - counting_down
    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;     
    // }

   
    // While loop
    // The while loop is used to execute a block of code as long as a condition is true.
    // let mut number = 3;
    // while number != 0 {
    //     println!("{}!", number);
    //     number -= 1;
    //     break;
    // }
    // println!("LIFTOFF!!!");

    // For loop
    // The for loop is used to iterate over a sequence of values.

    let a = [10, 20, 30, 40, 50, 60];
    let b = ["apple", "banana", "cherry", "date", "elderberry", "fig"];
    for element in a {
        println!("{element}");
    }
    for fruit in b {
        println!("{fruit}");
    }
}
