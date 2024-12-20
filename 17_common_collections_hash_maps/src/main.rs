// Hash Maps
// - A hash map allows you to associate a value with a particular key.
// - It implements a data structure called a hash table.
// - A hash table allows you to store and retrieve key/value pairs in O(1) time.
// - A hash table is implemented using an array of linked lists.
// - Each element in the array is a bucket.
// - Each bucket contains a key/value pair.
// - The key is used to calculate a hash value.
// - The hash value is used to determine the bucket where the key/value pair is stored.
// - The hash value is calculated using a hash function.
// - The hash function converts the key into a hash value.

use std::collections::HashMap;

fn main() {
//--------------------------HASHMAPS--------------------------
    let mut scores = HashMap::new();


    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
