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

    use std::collections::HashMap;

    // simple building
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yello"), 50);

    // inserting
    let field_name = String::from("color");
    let field_value = String::from("blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_* are invalid here because they've been moved into `map` by
    // insert().
    // also, it seems like rustc is able to figure out what type of data they
    // should each contain based on their first values?
    // println!("{}", field_name);              // won't work
    // map.insert(String::from("age"), 30i32);  // won't work

    // collecting
    let teams = vec![String::from("Blue"), String::from("Yello")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.iter().zip(initial_scores.iter()).collect();

    // teams.push(String::from("Green"));

    let score = scores.get(&String::from("Blue"));

    if let Some(&10) = score {
        println!("{:?}", score); // Some(10)
    }

    let team = String::from("Blue");
    scores.insert(&team, &25);

    for (key, value) in &scores {
        // not strictly in any order
        println!("{} has {}", &key, &value);
    }

    // udpdating based on old value
    let text = "hello world hello mom hello people people";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
