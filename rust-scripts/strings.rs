// Examples of strings.

fn main() {

    // String literal which is a string slice str or &str

    let s1 = "hello there";

    println!("s1 is {s1}");

    // String literal with emoji smiley face

    let s2 = "hello there 😀";

    println!("s2 is {s2}");

    // Loop through characters in a string

    let s3 = "hello";

    for c in s3.chars() {
        println!("char is {c}");
    }

    // Get the second character in a string

    let s4 = "strawberry";

    println!("second char is {}", s4.chars().nth(1).unwrap());

    // Create a String type from a string literal

    let s5 = String::from("hello again");

    println!("s5 is {s5}");

    // Create empty String type and append to it

    let mut s6 = String::new();
    s6.push_str("one");
    s6.push_str("two");

    println!("s6 is {s6}");

    // Use the format! macro to combine strings

    let a = String::from("tic");
    let b = String::from("tac");
    let c = String::from("toe");

    let s = format!("{a}-{b}-{c}");

    println!("s is {s}");
}