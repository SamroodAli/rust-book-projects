struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn add_width(&mut self, width: u32) {
        self.width += width;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut rect1 = Rectangle {
        height: 30,
        width: 50,
    };

    let rect2 = Rectangle {
        height: 20,
        width: 40,
    };

    println!(
        "Rectangle 1 is able to hold Rectangle 2: {}",
        rect1.can_hold(&rect2)
    );

    rect1.add_width(40);
    println!("The area of the rectangle is {}", rect1.area());
}
