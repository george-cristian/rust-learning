#![allow(dead_code)]

use std::mem;

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

pub fn structurizare() {
    let p = Point {x: 3.0, y: 4.0};
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point {x: 5.0, y: 10.0};
    let my_line = Line {start: p, end: p2};

    println!("my line starts at ({}, {}) and ends at ({}, {})", my_line.start.x, my_line.start.y, my_line.end.x, my_line.end.y);
}

pub enum Color
{
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    Cmyk {cyan:u8, magenta:u8, yellow:u8, black:u8}
}

pub fn enums(color:Color)
{

    match color {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0) => println!("black"),
        Color::Cmyk {cyan: _, magenta: _, yellow: _, black: 255} => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({},{},{})", r, g, b),
        _ => ()
    }
}

pub fn option(x:f64, y:f64)
{
    // Option contains either a Some() or None --> These are data types
    let result:Option<f64> =
        if y != 0.0 { Some(x/y) } else {None};

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("cannot divide {} by {}", x, y)
    }

    if let Some(z) = result { println!("z = {}", z); }
}

pub fn array()
{
    let mut a:[i32;5] = [1,2,3,4,5];
    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 420;
    println!("a has {} elements, first is {}", a.len(), a[0]);

    println!("{:?}", a);

    for i in 0..a.len() {
        println!("element {} is {}", i, a[i]);
    }

    let mut b = [1; 10];

    for i in 0..a.len() {
        println!("element {} is {}", i, b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx:[[f32; 3]; 2] = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];

    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            println!("element at row {} and col {} is {}", i, j, mtx[i][j]);
        }
    }
}

pub fn vectors() {
    //Vectors are dynamic

    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    a.push(34);
    println!("a = {:?}", a);

    // How to iterate through vectors
    for i in 0..a.len() {
        println!("element {} is {}", i, a[i]);
    }

    for x in &a {
        println!("alternative printing: {}", x);
    }

    // How to safely get an element of an vector
    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("this element does not exist")
    }

    // Removing elements from a vector
    let last_element = a.pop(); // This returns an Option<T>
    match last_element {
        Some(x) => println!("The last element of the vector is: {}", x),
        None => println!("There is no last element")
    }

    while let Some(x) = a.pop() {
        println!("x = {}", x);
    }

}