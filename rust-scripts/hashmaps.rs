// Examples of hash maps.

use std::collections::HashMap;

fn main() {

    // Create a new hash map, add elements to it, then get a value

    let mut scores = HashMap::new();
    scores.insert("first", 10.5);
    scores.insert("second", 34.09);

    println!("scores are {:?}", scores);
    println!("first score is {}", scores.get("first").copied().unwrap_or(0.0));

    // Iterate over each key/value pair in the hash map

    let mut cars = HashMap::new();
    cars.insert("ford", 1975);
    cars.insert("chevy", 1988);
    cars.insert("jeep", 1967);

    for (key, value) in cars {
        println!("key is {key}, value is {value}");
    }
}