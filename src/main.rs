use std::io;
use rand::Rng;

fn guess(mut g: &str, num: &u8, remaining: &mut u8) {
    println!("Please input your next guess");
    print!(" >> ");
    let mut gnum: u8;
    {
        io::stdin()
            .read_line(&mut g.to_owned())
            .expect("Error in reading line.");
        gnum = g.parse::<u8>().unwrap();
    }
    if gnum == *num {
        println!("Correct!\nThe number was: {}.\nYou got that in: {} guesses ({} guesses remaining).", num, 8 - *remaining, *remaining);
    } else {
        *remaining = *remaining - 1;// for some reason the -- and ++ operators dont exist in rust
        println!("Incorrect. {} guesses remaining.", remaining);
        if *num > gnum { println!("The number is higher than your guess."); } else { println!("The number is lower than your guess."); }
        for i in 0..3 { println!(); }
        guess(g, num, remaining);
    }
}

fn main() {
    println!("Hello, welcome to the guessing game! You have 8 guesses. \nThe number will be any integer from 0 to 100");
    let num: u8 = rand::thread_rng()
        .gen_range(0, 101);    
    let mut remaining: u8 = 8;
    let mut g = String::new();
    guess(&g, &num, &mut remaining);
}
