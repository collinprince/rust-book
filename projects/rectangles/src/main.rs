#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        rect.width < self.width && rect.height < self.height
    }

    // associated function which takes in a size param and returns a square w/ sides of that size
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    println!("rect1 is {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect2 can fit in rect1? : {}", rect1.can_hold(&rect2));

    let square1 = Rectangle::square(10);
    println!("square1 fields: {:?}", square1);
}
