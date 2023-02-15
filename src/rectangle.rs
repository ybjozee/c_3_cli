#[derive(Debug)]
struct Rectangle {
    length: u32,
    breadth: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.breadth
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.breadth > other.breadth && self.length > other.length
    }

    fn square(size: u32) -> Self {
        Self {
            length: size,
            breadth: size,
        }
    }
}

pub fn print_statistics() {
    let rect1 = Rectangle {
        length: 30,
        breadth: 50,
    };

    println!("rect 1 is {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    let scale = 2;
    let rect2 = Rectangle {
        length: 50,
        breadth: dbg!(30 * scale),
    };
    dbg!(&rect2);
    println!("rect2 can hold rect1 : {}", rect2.can_hold(&rect1));
    println!("rect1 can hold rect1 : {}", rect1.can_hold(&rect2));
    dbg!(Rectangle::square(90));
}
