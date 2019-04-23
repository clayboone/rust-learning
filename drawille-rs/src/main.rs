extern crate drawille;

use drawille::Canvas;
use std::{thread, time};

fn main() {
    let mut canvas = Canvas::new(100, 20);
    canvas.line(2, 2, 80, 80);
    timeout();
    println!("{}", canvas.frame());
    canvas.line(2, 80, 80, 80);
    timeout();
    println!("{}", canvas.frame());
    canvas.line(2, 2, 2, 80);
    timeout();
    println!("{}", canvas.frame());

    canvas.clear();
    timeout();
    println!("{}", canvas.frame());
}

fn timeout() {
    let duration = time::Duration::new(1, 0);

    thread::sleep(duration);
}
