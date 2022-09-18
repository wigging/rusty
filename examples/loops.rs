
fn main() {

    // Use `loop` to execute code repeatedly until `break` is called.

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            let count = counter * 2;
            break count
        }
    };

    println!("The result is {result}");

    // Use `while` to run code while a condition is true.

    let mut number = 3;

    while number != 0 {
        println!("{number}...");

        number -= 1;
    }

    println!("Blast off!");

    // Use `for` to loop through each element in a collection.

    let numbers = [10, 20, 30, 40, 50];

    for n in numbers {
        println!("The number is: {n}");
    }

    // Use `for` to execute some code for a certain number of times.

    for i in 1..4 {
        println!("Number is {i}");
    }
}