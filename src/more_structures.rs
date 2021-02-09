#![allow(dead_code)]

pub fn slices() {
    let mut data = [1, 2, 3, 4, 5];

    use_slice(&data[1..4]); // --> YOU CAN PROVIDE A SUBSET OF THE ORIGINAL ARRAY (THIS IS A SLICE)

    use_mut_slice(&mut data[1..4]); // --> NOTICE THAT YOU HAVE TO EXPLICITLY SAY MUT WHEN PASSING A REFERENCE THAT YOU WANT TO CHANGE
                                            // --> EVEN THOUGH THAT REFERENCE IS ALREADY MUTABLE (THIS PUTS AN EMPHASIS ON SECURITY)
    println!("The array is now: {:?}", data);
}

pub fn use_slice(slice_var: &[i32]) { //--> THIS MEANS THAT WE ARE GETTING A PART OF AN I32 ARRAY
    println!("first element of the slice is {}; the length of the slice is {}", slice_var[0], slice_var.len());
}

pub fn use_mut_slice(slice_var: &mut[i32]) {
    slice_var[0] = 420; // --> THIS CHANGE WILL PERSIST BECAUSE WE ARE UPDATING THE ACTUAL MEMORY ADDRESS
}

pub fn strings() {

    // &str - first type of strings in Rust - you cannot do much with these (maybe static strings)
    let s = "Hello there!";

    // Iterating through the characters sequence
    for c in s.chars().rev() {
        println!("character: {}", c);
    }

    // Getting a particular index from a string
    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char);
    }

    // String - second type of strings in Rust - heap allocated
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char); // --> YOU CAN PUSH DIRECTLY CHAR VARS
        letters.push(','); // --> YOU CAN PUSH DIRECTLY CHARS
        letters.push_str(":"); // --> YOU CAN PUSH STRING SLICES
        println!("our string is now {}", letters);
        a = a + 1;
    }

    // SOME APIS TAKE STRINGS, OTHERS TAKE STRING SLICES, SO YOU HAVE TO GET USED TO IT

    // Converting a String to &str
    let converted:&str = &letters; // --> THESE ARE CALLED DE-REF CONVERSIONS

    // Concatenating a &str with a String is fine
    let z = letters + "abc";
    //let y = z + &letters;

    // Making a String from a str
    let mut abc = String::from("ala bala portocala");
    let mut def = "herpderp".to_string();

}

fn sum_and_product(x:i32, y:i32) -> (i32, i32) {

    return (x + y, x * y);

}

pub fn tuples() {
    // A TUPLE IS A COLLECTION OF SEVERAL VALUES FOR WHICH YOU DO NOT CREATE A STRUCT FOR
    // YOU CAN PUT MISMATCHING TYPES IN A TUPLE (WHICH IS DIFFERENT FROM AN ARRAY WHERE YOU HAVE ALL THE ELEMENTS OF THE SAME TYPE)
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("The sum is {}, and the product is {}", sp.0, sp.1); // --> NOTICE THE FUNKY REFERENCING MECHANISM

    // Destructuring a tuple - like in Python
    let (a, b) = sp;
    println!("a = {} and b = {}", a, b);

    // You can have a tuple with tuples

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);

    let foo = (true, 42.0, -1i8); // --> MAKING A TUPLE WITH MULTIPLE TYPES

    let meaning = (42, ); // --> MAKING A TUPLE WITH ONE ELEMENT
}

fn how_many(x:i32) -> &'static str {
    match x { // --> THE FIRST CASE THAT IS MATCHED GETS RETURNED
        0 => "no",
        1 | 2 => "one or two", // --> NOTICE THE VERTICAL BAR WHICH ACTS LIKE AN OR
        12 => "a dozen",
        z @ 9..=11 => "lots of", // --> NOTE THAT HERE I GIVE A NAME TO THIS RANGE, AND THEN I CAN USE IT INSIDE MY PROCESSING CODE FOR THIS PATTERN
        _ if (x % 2 == 0) => "some",
        _ => "a few"
    }
}

pub fn pattern_matching(point:(i32, i32)) {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    match (point) {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("y axis, x = {}", x),
        (x, y) => println!("x = {}, y = {}", x, y)
    }
}

struct Point<T> {
    x: T,
    y: T
}

struct ComplexPoint<T, V> {
    x: T,
    y: V
}

struct Line<T> {
    start: T,
    end: T
}

fn generics() {

    let point:Point<i32> = Point { x: 0, y: 1 };
    let point_2:Point<f64> = Point { x: 1.2, y: 3.2};
    let point_3:ComplexPoint<i32, f64> = ComplexPoint { x: 1, y: 3.2};

    //let myline = Line {start: point, end: point_2}; // --> THIS DOES NOT COMPILE BECAUSE START AND END ARE OF DIFFERENT TYPES
                                                    // --> ONE IS POINT I32 AND ONE IS POINT F64
}