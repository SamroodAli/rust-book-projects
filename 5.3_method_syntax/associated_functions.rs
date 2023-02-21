struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }

    fn area(&self) {
        println!("area is {}", self.width * self.height);
    }
}

fn main() {
    let square = Rectangle::square(30);

    square.area();
}
