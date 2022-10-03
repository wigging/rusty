// A basic struct

struct Point {
    x: i32,
    y: i32
}

// A tuple struct

struct PointTuple(i32, i32);

// A struct with methods (functions)

struct Shape {
    width: u32,
    height: u32
}

impl Shape {

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn volume(&self, v: u32) -> u32 {
        self.width * self.height * v
    }
}

fn main() {

    // Using a basic struct

    let p = Point {
        x: 13,
        y: 8
    };

    println!("x is {}, y is {}", p.x, p.y);

    // Using a tuple struct

    let pt = PointTuple(32, 16);

    println!("x is {}, y is {}", pt.0, pt.1);

    // Using a struct with methods

    let shape = Shape { width: 2, height: 3 };
    let area = shape.area();
    let volume = shape.volume(2);

    println!("Area is: {area}");
    println!("Volume is: {volume}");
}
