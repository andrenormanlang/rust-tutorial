// Approach 1
// Result<T, E> approach <Option<T> is a special case of Result<T, ()>>
// Result<T, E> is a type that represents either success (Ok) or failure (Err)

// Option 1
// enum Option<T> {// Define the generic result type
//     Some(T), // Some(T) represents the presence of a value
//     None, // None represents the absence of a value
// }

// Example Option<T>
fn divideOption(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}
// Option 2
// enum Result<T, E> {// Define the generic result type
//     Ok(T), // Ok(T) represents success and contains a value
//     Err(E), // Err(E) represents failure and contains an error value
// }

// Example Result<T, E>
fn divideResult(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    let result = divideOption(10.0, 3.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by zero"),
    }

    match divideResult(100.23, 73.98){
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),

    }
}
