fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // let mut user1 ...
//    user1.email = String::from("anotheremail@example.com");

    println!("user: {:?}", user1);

    let user2 = User {
//        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("user: {:?}", user2);
    //-----------------------------
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );


    let rect1 = Rectangle{width: 30, height: 50};

    println!(
        "The area of the rectangle {:#?} is {} square pixels.",
        rect1, area3(&rect1)
    );

    println!(
        "method: The area of the rectangle {:#?} is {} square pixels.",
        rect1, rect1.area()
    );

    println!("can hold {}", rect1.can_hold(&Rectangle{width: 5, height: 19}));
    println!("can hold {}", rect1.can_hold(&Rectangle{width: 15, height: 188}));



    println!("square: {:?}", Rectangle::square(5))



}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}