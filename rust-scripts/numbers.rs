use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("type is {}", type_name::<T>())
}

fn main() {

    // Signed integer types are i8 (8-bit), i16 (16-bit), i32 (32-bit), i64
    // (64-bit), and i128 (128-bit)
    println!("\n--- Signed integers ---");

    let a: i8 = -23;
    println!("\na is {a}");
    print_type_of(&a);

    let a: i32 = -41;
    println!("\na is {a}");
    print_type_of(&a);

    let a: i64 = -52;
    println!("\na is {a}");
    print_type_of(&a);

    // The default integer type is i32

    let a = 34;
    println!("\na is {a}");
    print_type_of(&a);

    // Unsigned integer types are u8 (8-bit), u16 (16-bit), u32 (32-bit), u64
    // (64-bit), and u128 (128-bit)
    println!("\n--- Unsigned integers ---");

    let a: u8 = 58;
    println!("\na is {a}");
    print_type_of(&a);

    let a: u32 = 81;
    println!("\na is {a}");
    print_type_of(&a);

    let a: u64 = 95;
    println!("\na is {a}");
    print_type_of(&a);

    // Floating point number types are f32 (single precision) and f64 (double precision)
    println!("\n--- Floating point numbers ---");

    let b: f32 = 12.809;
    println!("\nb is {b}");
    print_type_of(&b);

    // The default float type is f64

    let b = 34.1;
    println!("\nb is {b}");
    print_type_of(&b);

    // Use an underscore to visually separate a number

    let c = 9_800.25;
    println!("\nc is {c}");
    print_type_of(&c);
}