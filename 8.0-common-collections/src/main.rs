#[derive(Debug)]
struct Pets {
    animal: String,
    name: String,
    age: i32,
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

    let mut v: Vec<u32> = Vec::new();
    for _ in 0..20 {
        match v.len() {
            0 => v.push(0),
            1 => v.push(1),
            _ => v.push(v[v.len() - 1] + v[v.len() - 2]),
        }
    }

    println!("{:?}", v);
}
