#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    if rect1.width() {
        println!("The Rectangle has a nonzero width: it is {}", rect1.width);
    }
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.height * rectangle.width
// }
