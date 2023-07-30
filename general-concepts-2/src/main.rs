fn main() {
    let mut x = 5;
    println!("The value of x = {}", x);

    x = 10;
    println!("Updated value of x = {}", x);

    let y: u32 = "10".parse().unwrap();

    println!("{}", y);

    // Characters
    let c1 = 'a';
    let c2 = '\u{263A}';
    let c3 = '5';
    println!("c1 = {}, c2 = {}, c3 = {}", c1, c2, c3);

    // Tuples
    let tup = (4, "hello", true);
    let first_elem = tup.0;
    println!("{}", first_elem);

    // Arrays
    let arr = [1, 2, 3];
    let num1 = arr[2];

    println!("num1 = {}", num1);

    func_first();
    func_second(10, 20);

    /*

    Control Flow

    */

    // if-else

    let x = 5;
    if x == 5 {
        println!("x is 5");
    } else {
        println!("x is NOT 5");
    }

    // Loops

    // Loop

    let mut x = 0;
    loop {
        x += 1;
        println!("x = {}", x);
        if x == 3 {
            break;
        }
    }

    // while loop
    // NB: this approach may be error-prone if indices not accessed correctly - leading to panic situation
    let mut x = 0;

    while x < 3 {
        x += 1;
        println!("x = {}", x)
    }

    // for loop
    let arr = [10, 20, 30, 40];
    for item in arr.iter() {
        println!("value = {}", item * 2)
    }
}

// Function with parameters

fn func_first() {
    println!("The first func called: ")
}

fn func_second(x: i32, y: u8) {
    println!("The second function called... {}, {}", x, y)
}
