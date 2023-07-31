fn main() {
    // Varibles are immutable by default
    let x = 37;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    // consts are not allowed to use 'mut'
    #[allow(unused)]
    const MAX_POINTS: u32 = 100_100;

    // Tuple type
    let tup = (-5, -9, 1);
    // destructuring
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // Arrays - in Rust they have a fixed length
    let arr: [i64; 5] = [1, 2, 3, 4, 5];

    double_v1(1);
    double_v1(-1);
}

// Exercises

// Exercise 1

fn double_v1(n: i32) -> i32 {
    return n * 2;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_double_v1() {
        assert_eq!(double_v1(1), 2);
        assert_eq!(double_v1(-1), -2);
    }
}


// Exercise 2

fn double_v2(n: i32) -> i64 {
    let double_num_i32 = n * 2;
    let double_num_i64 = i64::from(double_num_i32);
    double_num_i64;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_double_v2(){
        assert_eq!(double_v2(1), 2)
    }
}

