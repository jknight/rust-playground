use std::mem;

fn main() {
    // let binding
    let a : u8 = 123; // unsigned 8 bits
    // a is immutable
    println!("Hello, world! {}", a);

    let mut b : i8 = 0; // mutable
    println!("b is {}", b);
    b = 42;
    println!("b is now {}", b);

    // The rust compiler figures out the variable type
    let c = 123456789;
    // 2021 edition:
    // i32 = 32bits = 4 bytes
    println!("c is {} and takes up {} bytes", c, mem::size_of_val(&c));
    //println!("C is {} and takes up {} bytes", c, mem::size_of(&c));

    // usize, isize
    let d : usize = 1;
    println!("d is {}, size is {} bytes", d, mem::size_of_val(&d));
    println!("d takes up {} bytes, {}-bit OS", mem::size_of::<usize>(), mem::size_of::<usize>() * 8);

    let e : char = 'x'; // single quotes, immutable, 32bit unicode char
    println!("e is {}, size is {} bytes", e, mem::size_of_val(&e));

    let f = 120.12;
    println!("f is {} and is {} bytes", f, mem::size_of_val(&f));

    let g:f32 = 2.5;
    println!("g is {} and is {} bytes", g, mem::size_of_val(&g));

    let h = false;
    println!("h is {} and is {} bytes", h, mem::size_of_val(&h));
}
