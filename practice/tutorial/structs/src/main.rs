#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area (&self) -> u32 {
        self.width * self.height
    }

    fn canhold (&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square (size: u32) -> Rectangle {
        Rectangle {width:size, height:size}
    }
}

fn main() {
    let rect1 = Rectangle {width:30, height:50};
    let rect2 = Rectangle {width:20, height:40};
    let rect3 = Rectangle::square(15);
    println!(
        "The area of the rect is {} pixels.", rect1.area()
    );
    println!(
        "Rect 1 can hold Rect 2: {}", rect1.canhold(&rect2)
    );
    println!(
        "Rect 2 can hold Rect 1: {}", rect2.canhold(&rect1)
    );
    println!(
        "Rect 3 is {:#?}", rect3
    );
    println!(
        "Rect 3 is {:?}", rect3
    );
}