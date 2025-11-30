use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Hello, world!");

    // Gueesing game

    println!("Guessing Game---");

    let secret_number=rand::thread_rng().gen_range(1..10);
    println!("The secret number is {secret_number}");
    
    loop{
        println!("Please enter your number:");
        let mut guess=String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        
        let guess:u32=match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            },
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number){
            Ordering::Less=>println!("Too Small!"),
            Ordering::Equal=>{
                println!("Correct Number Guessed");
                break
            },
            Ordering::Greater=>println!("Too Big!")
        }
    }



    //Calculations


    let mut x=7;
    let mut y=10;

    println!("{x}, {y} : {}",x+y);
    println!("{x}, {y} : {}",x-y);
    println!("{x}, {y} : {}",x*y);
    println!("{x}, {y} : {}",x/y);
    println!("{x}, {y} : {}",x%y);
    x=x-12;
    y=100;
    println!("{x}:{y}");
}
