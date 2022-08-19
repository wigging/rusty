---
title: Structs
---

A struct is a custom data type that contains related values. Below are some examples of creating structs and accessing the values.

```rust
// Regular struct

struct Point {
    x: i32,
    y: i32
}

let p = Point {
    x: 13,
    y: 8
};

println!("x is {}, y is {}", p.x, p.y);

// Tuple struct

struct PointTuple(i32, i32);

let pt = PointTuple(32, 16);

println!("x is {}, y is {}", pt.0, pt.1);
```
