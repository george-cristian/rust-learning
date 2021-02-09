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

fn is_even(x: u32) -> bool {
    return x % 2 == 0;
}

pub fn higher_order_functions() {
    let limit = 500;
    let mut sum = 0;

    for i in 0.. {
        let isq = i * i;

        if isq > limit {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }
    println!("loop sum = {}", sum);

    let sum2 =
        (0..).map(|x| { return x * x; })
             .take_while(|&x| { return x < limit; }) // --> NOTE THAT HERE YOU PASS A REFERENCE (THIS IS HOW TAKE_WHILE WORKS
             .filter(|x| { return is_even(*x); })// --> NOTE THAT HERE A REFERENCE IS PASSED BY DEFAULT AND YOU HAVE TO ADD A *
             .fold(0, |sum, x| { return sum + x; });
    println!("higher order function sum = {}", sum2);
}

trait Animal { // --> THIS IS A JAVA ABSTRACT CLASS

    fn create(name: &'static str) -> Self; // --> NOTE THAT THIS FUNCTION IS STATIC, AND IS A CONSTRUCTOR
                                           // --> SELF IS THE CLASS NAME

    fn name(&self) -> &'static str;

    fn talk(&self) { // --> DEFAULT IMPLEMENTATION
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {
    fn create(name:&'static str) -> Human {
        return Human { name: name };
    }

    fn name(&self) -> &'static str {
        return self.name;
    }

    fn talk(&self) {
        println!("{} says Hurr Durr", self.name);
    }
}

impl Animal for Cat {
    fn create(name:&'static str) -> Cat {
        return Cat { name: name };
    }

    fn name(&self) -> &'static str {
        return self.name;
    }

    fn talk(&self) {
        println!("{} says wHaT iS tHe PuRpOsE Of LiFe?", self.name);
    }
}

trait Summable<T> { // --> THIS IS LIKE AN INTERFACE IN JAVA
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> { // --> EVERY VECTOR OF I32s THAT I WILL CREATE FROM NOW ON WILL HAVE THIS SUMMABLE INTERFACE, THUS THE SUM FUNCTION
    fn sum(&self) -> i32 {
        let mut result:i32 = 0;

        for x in self {
            result += *x;
        }

        return result;
    }
}

pub fn traits() {

    let h = Human { name: "Gogu" };
    h.talk();

    let h2 = Human::create("Bobi");
    h2.talk();

    let c = Cat { name: "Alfoncina" };
    c.talk();

    let c2 = Cat::create("Kush");
    c2.talk();

    //let a = Animal::create("Geon"); // --> THIS DOES NOT COMPILE BECAUSE YOU CANNOT INSTANTIATE AN ABSTRACT CLASS WITHOUT SPECIFYING A TYPE
    let a2:Human = Animal::create("Jucu"); // --> THE IMPLEMENTATION OF THE CREATE FUNCTION IS CHOSEN AT COMPILE TIME DEPENDING ON THE TYPE
    a2.talk();

    let vectorash = vec![1, 2, 3]; // --> NOTE THAT ALL VECTORS WHICH I WILL CREATE AS I32 VECTORS WILL HAVE THE SUMMABLE INTERFACE WHICH I IMPLEMENTED AT THE TOP, THUS THEY WILL HAVE THE SUM FUNCTION
    println!("sum = {}", vectorash.sum());
}