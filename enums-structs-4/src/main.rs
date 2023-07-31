fn main() {
    // Ownership

    // literals are immutable
    let _st = "hello";

    // String is
    let s1 = String::from("ken");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // Stack-Only Data: copy

    let x = 10;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // Ownership and Functions

    let name = String::from("hello");
    takes_ownership(name);

    let num = 5;
    makes_copy(num);

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // drop is called. Backing memory is freed

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // some_integer goes out of scope.Nothing special happens

fn calculate_length(s: &String) -> usize {
    s.len();
}
