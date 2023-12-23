use std::io;
use std::cmp::*;
use rand::Rng;

fn main() {
    println!("Hello, welcome to the guessing game! You have 8 guesses. \nThe number will be any integer from 0 to 100");
    let num: u8 = rand::thread_rng()
        .gen_range(0, 100);
    let mut remaining: u8 = 8;

    while remaining > 0 {
        println!("Please input your next guess.\n {} guesses remaining.", remaining);
        print!(" >> ");

        let mut g = String::new();
        io::stdin()
            .read_line(&mut g)
            .expect("Error in reading line");
        g.truncate(g.len() - 1);// remove newline

        let g: u8 = match g
            .trim()
            .parse()
            {
                Ok(n) => n,
                Err(_) => continue,
        };

        match g.cmp(&num) {
            Ordering::Less => {
                println!("The number is greater than your guess (your guess {})", g);
                remaining = remaining - 1;
                for _i in 0..2 {println!();}// don't know how to not use a variable for the loop
            },
            Ordering::Equal => {
                println!("Congratulations, {} was correct!", num);
                println!("You did that in {} guesses! ({} guesses remaining)", 8 - remaining, remaining);
                break;
            },
            Ordering::Greater => {
                print!("The number is less than your guess (your guess: {})", g);
                remaining = remaining - 1;// for some reason the ++ and -- operators dont exist in rust
                for _i in 0..2 {println!();}
            }
        }
    }
}
