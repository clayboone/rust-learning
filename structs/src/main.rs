// normal struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// struct tuples
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// w/ help from stackoverflow. this isn't part of the book (yet?)
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }
}

fn print_user(user: &User) {
    println!(
        "{} ({}) is {}signed in (total {}).",
        user.username,
        user.email,
        if user.active { "" } else { "not " },
        user.sign_in_count
    );
}

fn build_user(email: String, username: String) -> User {
    let template_user = User {
        username: String::new(),
        email: String::new(),
        active: true,
        sign_in_count: 1,
    };

    User {
        email,    // field init shorthand because variable names are the same
        username, // as field names

        ..template_user // field update syntax because why not
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("some.user@example.com"),
        username: String::from("someuser123"),
        sign_in_count: 123,
        active: true,
    };
    user1.username = String::from("someChangeUser321");
    print_user(&user1);

    let user2 = build_user(
        String::from("amark@example.com"),
        String::from("Mark Anthony"),
    );
    print_user(&user2);

    let black = Color(0, 0, 0);
    let blue = Color(0, 0, 255);
    let origin = Point(0, 0, 0);

    println!("point {} is color {}", origin, blue);
}
