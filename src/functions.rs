#![allow(dead_code)]

fn print_value(x:i32) {
    println!("value = {}", x);
}

fn increase(x:&mut i32) {
    *x += 1;
}

fn product(x:i32, y:i32) -> i32 {
    return x * y;
}

pub fn functions() {
    print_value(5); // --> NOTE THAT THE VALUE OF 5 GOES ON THE STACK

    let mut z = 1;

    increase(&mut z);
    println!("z = {}", z);

    let a = 3;
    let b = 5;
    let p = product(a, b);
    println!("{} * {} = {}", a, b, p);
}

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

impl Line { // --> THIS IS HOW YOU ADD BEHAVIOR TO STRUCTS (USING IMPL BLOCKS)
    fn len(&self) -> f64 { // --> NOTE THE REFERENCE TO SELF (LIKE IN PYTHON)
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        return (dx*dx + dy*dy).sqrt();
    }
}

pub fn methods() {
    let p = Point { x: 3.0, y: 4.0};
    let p2 = Point { x: 10.0, y: 1.0};
    let myline = Line { start: p, end: p2};

    println!("length of myline is = {}", myline.len());
}