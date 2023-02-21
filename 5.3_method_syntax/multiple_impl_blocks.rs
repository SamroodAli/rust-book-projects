struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 40,
        width: 40,
    };

    println!("The area of rectangle 1 is {}", rect1.area());

    let rect2 = Rectangle {
        height: 30,
        width: 30,
    };

    println!("rect1 can hold rectangle 2 ? {}", rect1.can_hold(&rect2));
}
