//Structs allow you to group related data together
//This is useful for creating complex data types that represent real-world entities

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
//calling a method to a object directly or calling a method to a pointer to an object has same syntax cause of concept automatic referencing and dereferencing

impl Rectangle {
    //Implmementation block to define methods for Rectangle struct
    //The first argument is always self which represents the instance of the struct
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //In implementation blocks, we can also define associated functions that are not methods
    //associated functions are functions that are associated with a struct but do not take self as a parameter
}

//One struct is able to have multiple implementation blocks
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    //We have to make the entire struct mutable to change any of its fields
    let mut user1 = User {
        email: String::from("yashmanek2001@gmail.com"),
        username: String::from("yashmanek"),
        active: true,
        sign_in_count: 1,
    };
    let name = user1.username;
    user1.username = String::from("newusername");
    println!("Username: {}", user1.username);

    let user2 = build_user(String::from("xyz@gmail.com"), String::from("xyzuser"));

    let user3 = User {
        email: String::from("abc@gmail.com"),
        username: String::from("abcuser"),
        ..user2 //struct update syntax
    };

    //Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect:{:#?}", rect);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let can_hold = rect1.can_hold(&rect2);
    println!("Can rect1 hold rect2? {}", can_hold);

    let rect3 = Rectangle::square(20);
    println!("Square rectangle: {:#?}", rect3);
}

fn build_user(email: String, username: String) -> User {
    User {
        email, //field init shorthand syntax
        username,
        active: true,
        sign_in_count: 1,
    }
}
