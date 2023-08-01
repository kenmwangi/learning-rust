fn main() {
    // Simple statement
    println!("Hello world!");

    // characters
    println!("Size of a char: {}", std::mem::size_of::<char>());
    println!("Size of string containing 'a': {}", "a".len());
    println!("Size of string containing ' ': {}", "".len());

    let slice = "Hello";

    println!("{:?}", "a".as_bytes());
    println!("Slice is {} bytes.", slice.len());
    let slice2 = "안녕!";
    println!("Slice2 is {} bytes", slice2.len());

    // integers - whole numbers without fractions
    let small_number = 10u8;
    let big_number = 100_000_000_i32;
    let number = 0_______u8;
    let number2 = 1____6____7___9i32;
    println!("{}, {}", number, number2);

    let my_number = 9;
    println!("This is new capturing of variables since 2021 {my_number}");

    // Floats
    let my_float = 5.0;
    let another_float = 4e-5f32;

    let sum_float = my_float + another_float as f64;
    println!("Sum_float is: {}", sum_float);

    // Functions
    println!("Hello number {}!", num_func());

    multiply(10, 54);
}

fn num_func() -> i32 {
    10
}

fn multiply(number_one: i32, number_two: i32) {
    let result = number_one * number_two;
    println!("{} times {} = {}", number_one, number_two, result)
}
