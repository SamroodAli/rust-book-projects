struct Rectangle {
    width: u32,
}

impl Rectangle {
    fn width(&self) -> u32 {
        self.width
    }
}

fn main() {
    let rect1 = Rectangle { width: 30 };

    println!(
        "The value of rectangle's width is {}, which is being returned in width() {}", // this is actually not a getter but just a method with the same name
        rect1.width,
        rect1.width()
    );
}
