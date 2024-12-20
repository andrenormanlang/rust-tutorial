#![allow(warnings)]

// Structs are used to create custom data types
// Structs are used to name and package related values similar to tuples

fn main() {
    // tuple
    let rect = (200, 500);

    // struct
    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // Creating an instance of the struct
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@test.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("user2@test.com");
    println!("User1 email: {}", user1.email);

    // Return struct from a function
    fn build(email: String, username: String) -> User {
        User {
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
    }

    // Creating an instance from another instance
    let user2 = User {
        email: String::from("user2@test.com"),
        ..user1
    };
    println!("User2: {:?}", user2);

    // Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    // Unit-like structs
    struct AlwaysEqual;
    let subject = AlwaysEqual;


}

