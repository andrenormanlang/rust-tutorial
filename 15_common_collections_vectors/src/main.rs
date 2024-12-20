//1 - Vectors (Vec<T>)

// A vector is a dynamic array. It can grow or shrink in size as needed. It is a generic type, so it can store any type of data. Vectors are created using the vec! macro. The macro takes a list of elements and returns a Vec<T> containing those elements. The type of the vector is inferred from the elements in the list. Here is an example:
// let v = vec![1, 2, 3, 4, 5];
// println!("The third element of v is {}", v[2]);
// println!("The length of v is {}", v.len());
//  Vectors can only store elements of the same type. If you try to create a vector with elements of different types, the compiler will give you an error. Here is an example:
// let v = vec![1, "two", 3, "four", 5];

fn main() {
    // let _v:Vec<i32> = Vec::new();
    // let mut _v: Vec<i32> = Vec::new();
    // let mut _v: Vec<i32> = vec![1, 2, 3];

    // _v.push(4);
    // _v.push(5);
    // _v.push(6);
    // _v.push(7);
    // _v.push(8);
    // _v.push(9);

    // println!("{:?}", _v);
    
    let _v = vec![1, 2, 3, 4, 5];

    // Takes a reference and not ownership to the element at index 2
    // let third: &i32 = &_v[2]; //Direct indexing
    // println!("The third element of v is {}", third);

    // get() method returns an Option<&T>
    let third = _v.get(2); //Using get() method
    match third {
        Some(third) => println!("The third element from a GET method for v is {}", third),
        None => println!("There is no third element."),
    }
}
