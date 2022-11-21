
fn main() {

    // Tuple with a variety of types
    let tup: (i8, f64, u16) = (12, 4.05, 5);

    // Access a tuple element using a period
    let first_item = tup.0;
    println!("First item is {first_item}");

    // Get all values out of a tuple
    let tup = (12, 54, 102);
    let (a, b, c) = tup;
    println!("a is {a}, b is {b}, c is {c}");
}
