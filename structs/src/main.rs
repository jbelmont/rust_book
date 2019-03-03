fn main() {
    let mut user1 = User {
        email: String::from("somebody@example.com"),
        username: String::from("somebody"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("another@example.com");

    let user2 = User {
        email: String::from("whatevs@example.com"),
        username: String::from("anotherone"),
        ..user1
    };

    let black = Color(0,1,2);
    let origin = Point(0,0,0);
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

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