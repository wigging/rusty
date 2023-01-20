+++
title = "Enums"
date = 2022-11-14
+++

Example of defining and using an enum (enumeration).

```rust
enum Fruit {
    Apple,
    Orange,
    Lemon,
    Mellon,
}

fn selection(fruit: Fruit) {
    match fruit {
        Fruit::Apple => println!("Selected the Apple fruit"),
        Fruit::Orange => println!("You chose the Orange fruit"),
        Fruit::Mellon => println!("You selected a Watermelon"),
        Fruit::Lemon => println!("Selected a Lemon fruit"),
    }
}

fn main() {
    let item1 = Fruit::Orange;
    selection(item1);

    let item2 = Fruit::Apple;
    selection(item2);

    let item3 = Fruit::Mellon;
    selection(item3);

    let item4 = Fruit::Lemon;
    selection(item4);
}
```

The output from the above example is shown below.

```
You chose the Orange fruit
Selected the Apple fruit
You selected a Watermelon
Selected a Lemon fruit
```
