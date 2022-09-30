/*
Examples of slicing strings and arrays.
*/

fn main() {

    // Example 1
    let s = String::from("hello there");
    let firstword = &s[0..5];
    let secondword = &s[6..11];
    println!("phrase is: {s}\n");
    println!("first word is: {firstword}");
    println!("second word is: {secondword}\n");

    // Example 2
    let firstword = &s[..5];
    let secondword = &s[6..];
    println!("first word is: {firstword}");
    println!("second word is: {secondword}\n");

    // Example 3
    let items = [1, 2, 3, 4, 5];
    let slice = &items[1..4];
    println!("items are: {:?}", items);
    println!("items slice is: {:?}", slice);
}