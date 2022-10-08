// use std::String;

#[derive(Debug)]
struct Person {
    name: String,
    yob: i32,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn main() {
    let family: Vec<Person> = vec![
        Person {
            name: String::from("Oto"),
            yob: 1987,
        },
        Person {
            name: String::from("Martina"),
            yob: 1988,
        },
        Person {
            name: String::from("Tinkara"),
            yob: 2016,
        },
        Person {
            name: "Rudi".to_string(),
            yob: 2019,
        },
        Person {
            name: "Frida".to_string(),
            yob: 2020,
        },
    ];

    for member in &family {
        println!("Hello, {}", member)
    };

    println!("Size: {:?}", family.len());
    for (i, m) in family.iter().enumerate() {
        println!("Hey, {}, {}", m, i)
    }

    let numbers: Vec<i32> = (0..10).collect();
    for i in numbers {
        print!("{}", i)
    }
    println!();
}
