
fn main() {

    // Pass a reference of s1 to the calculate_length function. The &s1 refers
    // to the value of s1 without owning it.

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {s1} is {len}");
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}