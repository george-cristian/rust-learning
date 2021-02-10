#![allow(dead_code)]
#![allow(unused)]

pub fn ownership() {
    let v = vec![1, 2, 3]; // --> The variable v owns the memory of the vector
                                    // --> v is on the stack, but the data is on the heap

    //let v2 = v; // --> This basically copies the pointer
                         // --> Rust figures out that this can introduce a race condition
                         // --> This will invalidate the variable v

    let foo = |v:Vec<i32>| ();
    //foo(v); // --> This does not compile, because v is invalidated by v2

    //println!("{:?}", v); // --> This does not compile, because v is invalidated by v2

    let u = 1; // --> This is a primitive type that is put on the stack
    let u2 = u; // --> This will copy the entire value (not move, like with pointers)
    println!("u = {}", u); // --> This will work because with primitive types (like i32), the value is completely copied
}

pub fn borrowing() {

    let print_vector = |x:&mut Vec<i32>| { // --> Notice that the argument is a reference type --> This will enable the function to borrow the value instead of taking control of it
        println!("{:?}", x);
        x.push(69);
        println!("after push = {:?}", x);
    };

    let mut v = vec![3, 2, 1];

    print_vector(&mut v); // --> Instead of passing control of the vector v to the print_vector function, this will let print_vector to borrow the vector for a while

    println!("{:?}", v);

    let mut a = 40;
    {
        let b = &mut a; // --> Here b will borrow a as a mutable reference
        *b += 69;
    } // --> If this scope was not here, the variable b would not have been released, so a would still be borrowed by a mutable reference
    println!("a = {}", a);


}