#![allow(dead_code)]
#![allow(unused)]

extern crate rand;
extern crate grootings;

use rand::Rng;
use grootings::greetings::french;

pub fn my_crate() {
    println!("English: {}, {}",
             grootings::greetings::english::hello(),
             grootings::greetings::english::goodbye());

    println!("French: {}, {}",
             french::hello(),
             french::goodbye());
}

pub fn random() {
    let mut rng = rand::thread_rng();
    let b:i32 = rng.gen();

    println!("b = {}", b);
}

