mod sound {
    const MY_CONST: u32 = 42;
    pub const MY_PUB_CONST: u32 = 9001;

    pub fn guitar() {
        println!("guitar()");
        instrument::woodwind::clarinet();
    }

    pub mod instrument {
        pub mod woodwind {
            pub fn clarinet() {
                println!("clarinet()");
            }
        }
    }

    pub mod voice {
        pub fn voice() {
            println!("voice({})", super::MY_CONST);
            super::guitar();
        }
    }

}

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }

        pub fn id(&self) -> i32 {
            self.id
        }
    }
}

pub mod drinks {
    pub enum Soda {
        Fanta,
        Sprite,
    }
}

mod menu {
    pub use crate::drinks;

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
        Fanta,
    }

}

fn main() {
    println!("Sounds --------");
    use sound::*;
    voice::voice();
    println!("{}", sound::MY_PUB_CONST);

    println!("Plants --------");
    let mut v = plant::Vegetable::new("squash");
    v.name = String::from("yallow squash");
    println!("name: {}", v.name);

    // the `id` field is private, but we can use the id() method to get it
    // println!("id: {}", v.id);
    println!("id: {}", v.id());

    println!("Menu --------");
    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;
    let order3 = menu::Appetizer::Fanta;
    println!("order1: {:?}", order1);
    println!("order2: {:?}", order2);
    println!("order3: {:?}", order3);
}
