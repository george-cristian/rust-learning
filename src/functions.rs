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

fn say_hello() { println!("Hello!"); }

pub fn closures() {
    let sh = say_hello; // --> YOU CAN ASSIGN FUNCTIONS TO VARIABLES (AS IN PYTHON)
    sh();

    let plus_one = |x:i32| -> i32 { return x + 1; }; // --> THESE ARE INLINE FUNCTIONS
                                                                // --> THIS WILL EXIST ONLY INSIDE THE CLOSURES FUNCTION
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let mut two = 2;

    {
        let plus_two = |x|
            {
                let mut z = x;
                z += two; // --> NOTICE THERE IS A DEPENDENCY ON THE VARIABLE TWO HERE. THIS WILL RENDER IT NOT BORROWABLE AFTER THIS FUNCTION.
                return z;
            };
        println!("{} + 2 = {}", 3, plus_two(3));
    } // --> THIS SCOPE HELPS TO DESTROY THE plus_two FUNCTION, SO THAT THE VARIABLE TWO CAN BE BORROWED

    let borrow_two = &mut two; // --> THIS DOES NOT COMPILE (WHEN plus_two IS NOT SCOPED) BECAUSE TWO IS ALREADY USED INSIDE THE plus_two FUNCTION
    println!("two is equal to {}", borrow_two);

    let plus_three = |x:&mut i32| { *x += 3; }; // --> THE ARGUMENT IS PASSED BY A MUT REFERENCE SO I CAN DE-REF IT AND INCREASE IT
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);
}