#[derive(Debug)]
struct Pets {
    animal: String,
    name: String,
    age: i32,
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let p = vec![
        Pets {
            animal: String::from("Cat"),
            name: String::from("Bob"),
            age: 4,
        },
        Pets {
            animal: String::from("Dog"),
            name: String::from("Sudo"),
            age: 10,
        },
    ];

    println!("{:?}", p);

    let mut v: Vec<f64> = Vec::new();
    for _ in 0..20 {
        match v.len() {
            0 => v.push(0f64),
            1 => v.push(1f64),
            _ => v.push(v[v.len() - 1] + v[v.len() - 2]),
        }
    }

    println!("{:?}", v);

    // `i` won't be a copy of the element of the arry, it's a reference to it.
    // So this will modify the array in-place.
    for i in &mut v {
        // i *= 1.618; // Need to deference the reference to `&mut` first.
        *i *= 1.618;
    }

    println!("{:?}", v);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("hello there")),
    ];

    println!("{:?}", row);

    let mut _s1 = String::from("foo");
    let mut _s2 = " bar".to_string();
    println!("{}", _s1 + &_s2);
    // println!("{}", _s2 + &_s2); // _s2 was moved in prev line

    let s = String::from("Здравствуйте");
    println!("{}", s.len());
    println!("{}", &s[0..4]);
}
