#![allow(dead_code)]

use std::mem;

struct Point
{
    x: f64,
    y: f64
}

fn origin() -> Point
{
    Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap()
{
    let p1 = origin(); //stack allocated

    let p2 = Box::new(origin()); //heap allocated

    println!("p1 takes {} bytes", mem::size_of_val(&p1)); //this is the total size of the structure (8 + 8 = 16)
    println!("p2 takes {} bytes", mem::size_of_val(&p2)); //this is the size of the address, which is 64 bit on my laptop
    println!("p2 dereferenced takes {} bytes", mem::size_of_val(&(*p2))); //this is the size of the variable itself (it will print the same
                                                                                // thing as for p1

}