use core::fmt;

fn main() {
    println!("Hello, world!");

    const THIS_IS_HOW_IT_IS_WRITTEN: u32 = 100; // type is must for const keyword
    println!("This is how it is written for constant values: {THIS_IS_HOW_IT_IS_WRITTEN}");

    //interview question
    let x = 10;
    let x = x + 1; //global scope
    {
        let x = x + 10; //local scope
        println!("{x}");
    }
    println!("{x}");

    //This will work
    let x = "   ";
    let x = x.len();

    //This won't, will give compile time error
    let mut x = "  ";
    //  x=x.len();

    //  Rust: statically typed language, all variables must be defined with its type

    // two Types: Scalar & Compound (Group of Scalar)

    let tup: (i32, f32, u32) = (10, 2.3, 40);

    let first_val = tup.0;
    let second_val = tup.1;
    let third_val = tup.2;

    println!("{first_val} {second_val} {third_val}");

    let a = [12, 1, 3];

    for x in a {
        print!("{x} ");
    }
    println!("");
    let m = another_function();
    println!("{m}");
    let y = {
        let x = 10;
        x + 1
    };
    println!("Value of y is {y}");

    // Control Flow

    if y < 10 {
        println!("y less than 10");
    } else {
        println!("y greater than 10");
    }
}

fn another_function() -> i32 {
    println!("Checking if things work ");
    10
}
