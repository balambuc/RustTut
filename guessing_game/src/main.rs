extern crate rand;

use std::io::stdin;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("     Guessing Game!");
    let num = rand::thread_rng().gen_range(1,101);
    loop {
        println!("Enter a guess: ");

        let mut guess = String::new();
        stdin().read_line(&mut guess)
            .expect("ERROR");

        let guess :u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("NaN!");
                continue;
            }
        };

        match guess.cmp(&num) {
            Ordering::Less      => println!("Small"),
            Ordering::Greater   => println!("Big"),
            Ordering::Equal     => {
                println!("Just right");
                break;
            }
        }

    }

}
