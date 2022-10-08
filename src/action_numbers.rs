use std::rc::Rc;
use std::string::String;
use std::sync::{Arc, Mutex};

fn add(i: i32, j: i32) -> i32 {
    i + j
}

pub fn main() {
    println!("Numbers, yey!");

    let a = 10;
    let b = Box::new(20);
    let c = Rc::new(Box::new(30));
    let d: Arc<Mutex<i32>> = Arc::new(Mutex::new(40));

    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);

    let dvar = Arc::clone(&d);
    let dvar2 = Arc::try_unwrap(d).unwrap_err();
    println!("X = {:?}, {:?}", dvar, dvar2);

    println!("Sum = {:?}", add(add(add(a, *b), **c), 42));

    let e: i32 = 10;
    let f: u16 = 100;
    let f_t: i32 = f.try_into().unwrap();
    println!("e + f = {:?}", e + (f as i32) + f_t);

    let person: (String, String, i32) = ("Oto".to_string(), "Brglez".to_string(), 1987);
    println!("Person {:?}, {:?}", person, person.0)
}
