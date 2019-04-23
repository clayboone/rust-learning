fn main() {
    let number = 3;

    if number > 5 {
        println!("condition expression is true");
    } else {
        println!("condition expression is false");
    }

    // Works, but consider using match()
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition: bool = true;
    let number = if condition { 5 } else { 6 }; // rust has no ternary operator

    println!("number is {} because condition is {}", number, condition);

    println!("loop_counter() is {}", loop_counter());

    println!("while_counter() is {}", while_counter());

    for_counter();

    reverse_for_counter();
}

fn loop_counter() -> i32 {
    let mut counter = 0;

    // This, the final expression in the function, is what gets returned.
    // let result = loop {
    loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }

    // return counter and/or result
}

fn while_counter() -> i32 {
    let mut number = 3;

    while number > 0 {
        println!("{}!", number);

        number -= 1;
    }

    number
}

fn for_counter() {
    // let a = [10; 5]; // -> [10, 10, 10, 10, 10]
    let a = [10, 20, 30, 40, 50];

    for element in a.iter().rev() {
        println!("the value is {}", element);
    }
}

fn reverse_for_counter() {
    for number in (1..4).rev() {
        println!("1 to 4: {}", number);
    }

    // 4..0 -> compiler warning about decrementing...? use .rev().
    for number in (0..4).rev() {
        println!("0 to 4: {}", number);
    }
}
