fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    println!("five() is {}", five());
    println!("plus_one(5) is {}", plus_one(5));
}

/**
 * Another function with 2 parameters.
 */
fn another_function(x: i32, y: i32) {
    println!("x is {}, y is {}", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
