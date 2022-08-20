
fn main() {

    // Create an array of integers and access the first item
    let a = [3, 4, 5, 6, 7, 8];
    let first_item = a[0];
    println!("First item in a is {first_item}");

    // Create an array with 3 elements of i64 and get the second item
    let b: [i64; 3] = [12, 8, 5];
    let second_item = b[1];
    println!("Second item in b is {second_item}");

    // Create an array of 4 elements where each element is 1
    let c = [1; 4];
    println!("The c array is {:?}", c);

    // Get the length (number of elements) of an array
    let d = [3, 4, 5, 6, 7];
    let length_of_d = d.len();
    println!("Length of d is {length_of_d}");
}
