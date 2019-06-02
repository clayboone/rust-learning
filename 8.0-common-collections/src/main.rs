#![allow(unused_variables)]
fn main() {
    let v: Vec<i32> = Vec::new();

    let mut v = vec![1, 1];
    v.push(2);
    v.push(3);
    v.push(5);

    println!("{:?}", v);
}
