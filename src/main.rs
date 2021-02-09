#![allow(dead_code)]
#![allow(unused)]

mod sh;
mod structures;
mod more_structures;
mod functions;

use std::mem::size_of_val;

const MEANING_OF_LIFE:u8 = 42;

static Z:i32 = 123;

fn scope_and_shadowing() {

    let a:u8 = 123; // 8bits

    println!("a = {}", a);

    //a = 456;

    let mut b:i8 = 0;
    println!("b = {}", b);

    b = 42;
    println!("b = {}", b);

    let mut c = 1234567;
    println!("c = {}, size = {} bytes", c, size_of_val(&c));

    c = -1;
    println!("c = {} after modification", c);

    let z:isize = 123;
    let size_of_z = size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    let d:char = 'x';
    println!("d = {}, size = {} bytes", d, size_of_val(&d));

    let e = 2.5;
    println!("e = {}, size = {} bytes", e, size_of_val(&e));

    let g = false;
    println!("g = {}, size = {} bytes", g, size_of_val(&g));
}

fn operators()
{
    let mut a = 2 + 3 * 4;
    println!("a = {}", a);

    a = a + 1;
    a -= 2;

    println!(" remainder of {} / {} = {}", a, 3, (a % 3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_cubed_f = f64::powf(b, 2.5);
    println!("=======================");
    println!("b = {}", b);
    println!("b cubed int = {}", b_cubed);
    println!("b cubed float = {}", b_cubed_f);

    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("b to the power of PI is: {}", b_to_pi);

    // bitwise operators
    let c = 1 | 2;
    println!("1 OR 2 is {}", c);

    let two_to_10 = 1 << 10; //this is actually 2 to the power of 10 (because 2 at the power of 0 is 1, and if you shift it 10 places then it goes to 2 to the power of 10)
    println!("1 shifted 10 places is: {}", two_to_10);

    // logical operators
    let pi_less_t4 = std::f64::consts::PI < 4.0; //true
}

fn scope()
{
    let a = 123;

    {
        let b = 456;
        println!("inside, b = {}", b);

        let a = 777;
        println!("inside, a = {}", a);
    }

    println!("a = {}", a);
}

fn if_statement(temp:i32)
{
    //let temp = 35;

    if temp > 30
    {
        println!("It's really hot outside");
    } else if temp < 10{
        println!("Nope, it's not really hot, IT'S COLD!")
    } else {
        println!("It's fine bro");
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("Today is {}", day);

    println!("It is {}",
        if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"});
}

fn while_and_loop() {
    let mut x = 1;

    while x < 1000 {
        println!("x = {}", x);

        x = x * 2;
    }

    let mut y = 1;

    loop {
        y = y * 2;
        println!("y = {}", y);

        if y == 1<<10 {break;}
    }
}

fn for_loop() {

    for x in 1..11 {
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("pos = {}, value = {}", pos, y);
    }
}

fn match_statement(country_code:i32) {


    let country = match country_code {
        44 => "UK",
        46..=50 => "Sweden",
        7 => "Russia",
        _ => "Unknown"
    };

    let country_sec = match country_code {
        44 => "UK",
        46..=50 => "Sweden",
        7 => "Russia",
        1..=999 => "Unknown",
        _ => "Invalid"
    };

    println!("the country with code {} is {}", country_code, country);

}

fn main()
{
    //scope_and_shadowing();
    //operators();
    //scope();
    //sh::stack_and_heap();

    //if_statement(45);
    //while_and_loop();

    //for_loop();
    //match_statement(22);

    //structures::structurizare();
    //println!("=====================================");
    //structures::enums(structures::Color::RgbColor(1, 2, 3));
    //println!("=====================================");
    //structures::option(10.0, 1.0);
    //println!("=====================================");
    //structures::array();
    //println!("=====================================");
    //structures::vectors();


    println!("=========== MORE DATA STRUCTURES ===============");
    println!("=========== SLICES ===============");
    more_structures::slices();
    println!("=========== STRINGS ===============");
    more_structures::strings();
    println!("=========== TUPLES ===============");
    more_structures::tuples();
    println!("=========== PATTERN MATCHING ===============");
    more_structures::pattern_matching((0, 0));
}