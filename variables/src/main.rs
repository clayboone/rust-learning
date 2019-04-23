

#![allow(unused_variables)]
fn main() {
    let x = 5;
    println!("x is {}", x);
    let y = &x;
    let x = 6;
    println!("x is {}, y is {}", x, y);

    let mut a: i128 = 1;

    a <<= 1; // 0001 -> 0010
    a <<= 1; // 0010 -> 0100
    a += 1;  // 0100 -> 0101
    a <<= 1; // 0101 -> 1010 (10)

    println!("a is {}", a);

    let mut b: u8 = 2;
    let p: u32 = 7;

    b = b.pow(p) - 1; //  2**7 - 1 = 1000 0000 - 1 = 0111 1111
    b <<= 4; // 0111 1111 -> 1111 0000

    println!("b is {}", b);

    let c = 2 / 3; // integer division truncates, doesn't round.
    let d = 2 as f32 / 3 as f32;

    println!("c is {}", c);
    println!("d is {}", d);

    println!("2.1 * 3 is {}", 2.1 * 3 as f64);


    let guess: u32 = "42".parse()
        .expect("not a number");
}
