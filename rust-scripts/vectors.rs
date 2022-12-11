// Examples of vectors.

fn main() {

    // Create a vector of numbers

    let v1 = vec![3, 5, 8, 10, 12, 9];
    println!("v1 is {:?}", v1);

    // Create a vector of strings

    let v2 = vec!["one", "two", "three"];
    println!("v2 is {:?}", v2);

    // Add elements to an empty vector

    let mut v3 = Vec::new();
    v3.push(1.5);
    v3.push(3.0);
    v3.push(8.2);
    println!("v3 is {:?}", v3);

    // Read elements from a vector

    let v4 = vec![22, 38, 42, 50];
    let first = &v4[0];
    let third = v4[2];
    let last = v4.last().unwrap();

    println!("v4 is {:?}", v4);
    println!("first is {}", first);
    println!("third is {}", third);
    println!("last is {}", last);

    // Iterate over elements in a vector

    let v5 = vec![3, 4, 5, 6, 8, 12];

    for v in v5 {
        println!("v in v5 is {}", v);
    }

    // Iterate over elements in a vector by reference

    let v5 = vec![3, 4, 5, 6, 8, 12];

    for v in &v5 {
        println!("v in v5 is {}", v);
    }

}