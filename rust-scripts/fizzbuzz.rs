// Examples of the fizzbuzz problem written in Rust.

fn main() {

    // Example 1

    for i in 1..21 {
        print!("i is {} ", i);
        if i % 5 == 0 && i % 3 == 0 {
            println!("FizzBuzz");
        } else if i % 5 == 0 {
            println!("Fizz");
        } else if i % 3 == 0 {
            println!("Buzz");
        } else {
            println!("");
        }
    }

    // Example 2

    for i in 1..21 {
        print!("i is {} ", i);
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (_, 0) => println!("Fizz"),
            (0, _) => println!("Buzz"),
            _ => println!(""),
        }
    }

}