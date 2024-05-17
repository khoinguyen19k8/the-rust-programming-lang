struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Rectangle {
    width: u32,
    height: u32,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let email = String::from("someone@example.com");
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}
